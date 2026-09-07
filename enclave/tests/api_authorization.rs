//! API authentication and authorization tests for `enclave/src/server/`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §3   "REST / mTLS (Port 8080)" and "mTLS / RA-TLS (Port 8443)"
//!   §5.1 "Inter-node network communication uses RA-TLS to bind Intel DCAP
//!         attestation quotes directly into X.509 TLS certificates."
//!
//! ─────────────────────────────────────────────────────────────────────────
//! THE FINDING THIS FILE EXISTS FOR
//! ─────────────────────────────────────────────────────────────────────────
//!
//! `router::dispatch` matches (method, path) and calls the handler. It never
//! calls `HttpRequest::bearer_token()`. That helper is defined and has zero
//! callers in the entire crate. `EnclaveTokenService` is constructed in
//! `main.rs`, stored in `EnclaveState`, and used only to *issue* tokens —
//! never to gate a request.
//!
//! Every route is therefore unauthenticated, including:
//!
//!   GET  /v1/secrets/{id}          → returns the decrypted secret
//!   POST /v1/keys/{id}/sign        → signs arbitrary data with any key
//!   POST /v1/keys/{id}/decrypt     → decrypts arbitrary ciphertext
//!   POST /v1/tokens                → mints a token for any subject
//!   DELETE /v1/keys/{id}           → hard-deletes a key
//!
//! `EnclaveError::Unauthorized` and `EnclaveError::InsufficientScope` are
//! defined in `error.rs` and returned by no route in the crate.
//!
//! The host proxy compounds this: `host/src/main.rs` applies
//! `CorsLayer::permissive()`, which sends `Access-Control-Allow-Origin: *`.
//! Any web page the operator visits can script the full secrets API.
//!
//! See issues ENC-090 (no authn), ENC-091 (no authz), HOST-010 (CORS).
//!
//! These tests are written against the API the spec requires. They are
//! `#[ignore]`d because the enforcement layer does not exist yet — there is
//! nothing to call. Remove the ignores as each control lands.

mod common;

use common::{FixedKeyProvider, TempStore};
use traces_sm_enclave::auth::EnclaveTokenService;
use traces_sm_enclave::error::EnclaveError;

// ─────────────────────────────────────────────────────────────────────────────
// Token service — these run today
// ─────────────────────────────────────────────────────────────────────────────

/// TC-AUTH-001 — an issued token must verify.
#[test]
fn tc_auth_001_issued_token_verifies() {
    let tmp = TempStore::new("tok-verify");
    let p = FixedKeyProvider::new(0x31);
    let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");

    let (jti, jwt) = svc
        .issue_token("alice", vec!["secrets:read".into()], 3600, &p)
        .expect("issue");

    let claims = svc.verify_token(&jwt).expect("verify");
    assert_eq!(claims.sub, "alice");
    assert_eq!(claims.jti, jti);
    assert_eq!(claims.scopes, vec!["secrets:read".to_string()]);
}

/// TC-AUTH-002 — a tampered signature must be rejected.
#[test]
fn tc_auth_002_tampered_signature_is_rejected() {
    let tmp = TempStore::new("tok-tamper");
    let p = FixedKeyProvider::new(0x32);
    let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");
    let (_, jwt) = svc.issue_token("bob", vec![], 3600, &p).expect("issue");

    let mut parts: Vec<&str> = jwt.split('.').collect();
    let forged_sig = "A".repeat(parts[2].len());
    parts[2] = &forged_sig;
    assert!(svc.verify_token(&parts.join(".")).is_err());
}

/// TC-AUTH-003 — a tampered payload must be rejected.
#[test]
fn tc_auth_003_tampered_payload_is_rejected() {
    use base64::Engine as _;

    let tmp = TempStore::new("tok-payload");
    let p = FixedKeyProvider::new(0x33);
    let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");
    let (_, jwt) = svc
        .issue_token("carol", vec!["read".into()], 3600, &p)
        .expect("issue");

    let parts: Vec<&str> = jwt.split('.').collect();
    let forged = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(br#"{"sub":"root","iat":0,"exp":99999999999,"jti":"x","scopes":["admin"]}"#);
    let forged_jwt = format!("{}.{}.{}", parts[0], forged, parts[2]);
    assert!(
        svc.verify_token(&forged_jwt).is_err(),
        "a privilege-escalating payload swap was accepted"
    );
}

/// TC-AUTH-004 — an expired token must be rejected.
#[test]
fn tc_auth_004_expired_token_is_rejected() {
    let tmp = TempStore::new("tok-exp");
    let p = FixedKeyProvider::new(0x34);
    let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");
    let (_, jwt) = svc.issue_token("dave", vec![], 0, &p).expect("issue");

    std::thread::sleep(std::time::Duration::from_millis(1100));
    assert!(matches!(
        svc.verify_token(&jwt),
        Err(EnclaveError::TokenExpired)
    ));
}

/// TC-AUTH-005 — a revoked token must be rejected.
#[test]
fn tc_auth_005_revoked_token_is_rejected() {
    let tmp = TempStore::new("tok-revoke");
    let p = FixedKeyProvider::new(0x35);
    let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");
    let (jti, jwt) = svc.issue_token("erin", vec![], 3600, &p).expect("issue");

    assert!(svc.verify_token(&jwt).is_ok());
    svc.revoke_token(&jti);
    assert!(matches!(
        svc.verify_token(&jwt),
        Err(EnclaveError::TokenRevoked)
    ));
}

/// TC-AUTH-006 — a structurally malformed token must be rejected.
#[test]
fn tc_auth_006_malformed_tokens_are_rejected() {
    let tmp = TempStore::new("tok-malformed");
    let p = FixedKeyProvider::new(0x36);
    let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");

    for bad in ["", ".", "a.b", "a.b.c.d", "not-a-jwt", "...."] {
        assert!(
            svc.verify_token(bad).is_err(),
            "accepted malformed token {bad:?}"
        );
    }
}

/// TC-AUTH-007 — revocation must survive an enclave restart.
///
/// `EnclaveTokenService.revoked` is an in-memory `HashSet`. The `store_path`
/// field is captured in the struct and never read. Restart the enclave and
/// every revoked token is valid again, for the remainder of its TTL.
///
/// The host schema even has a `tokens(token_id, revoked)` table for this —
/// created by `db::init_db()` and never written to. See issue ENC-092.
#[test]
fn tc_auth_007_revocation_survives_restart() {
    let tmp = TempStore::new("tok-persist");
    let p = FixedKeyProvider::new(0x37);

    let (jti, jwt) = {
        let svc = EnclaveTokenService::new(tmp.path(), &p).expect("token service");
        let (jti, jwt) = svc.issue_token("frank", vec![], 3600, &p).expect("issue");
        svc.revoke_token(&jti);
        assert!(
            svc.verify_token(&jwt).is_err(),
            "revocation did not take effect"
        );
        (jti, jwt)
    };

    // Simulate an enclave restart against the same sealed store.
    let restarted = EnclaveTokenService::new(tmp.path(), &p).expect("restart");
    assert!(
        restarted.verify_token(&jwt).is_err(),
        "token {jti} was revoked before restart but verifies again after — \
         the deny-list is in-memory only (issue ENC-092)"
    );
}

/// TC-AUTH-008 — the JWT header must be validated.
///
/// `verify_token` splits on '.' and ignores `parts[0]` entirely. It never
/// parses the header or checks `alg`. Ed25519 verification is hard-coded, so
/// classic `alg: none` confusion is not directly exploitable — but an
/// unparsed, unvalidated header is a latent hazard the moment a second
/// algorithm is added, and it means `typ` and `crit` are unchecked too.
#[test]
#[ignore = "verify_token never parses the JWT header — see issue ENC-093"]
fn tc_auth_008_jwt_header_alg_is_validated() {
    unimplemented!("parse parts[0] and require alg == EdDSA, typ == JWT");
}

// ─────────────────────────────────────────────────────────────────────────────
// Route enforcement — none of this exists yet
// ─────────────────────────────────────────────────────────────────────────────

/// TC-API-001 — an unauthenticated request must be refused with 401.
///
/// THE headline security finding. Today every route in `router::dispatch`
/// executes regardless of the Authorization header.
#[test]
#[ignore = "dispatch() never calls bearer_token() — see issue ENC-090"]
fn tc_api_001_unauthenticated_request_is_refused() {
    unimplemented!(
        "add an auth gate to router::dispatch and assert 401 for every /v1/* \
         route with no Authorization header"
    );
}

/// TC-API-002 — a token without the required scope must be refused with 403.
#[test]
#[ignore = "no scope checking exists — see issue ENC-091"]
fn tc_api_002_insufficient_scope_is_refused() {
    unimplemented!(
        "map each route to a required scope (secrets:read, keys:sign, ...) \
         and assert InsufficientScope for a token that lacks it"
    );
}

/// TC-API-003 — the health endpoint must remain unauthenticated.
///
/// Deliberate carve-out so the auth gate is written as an allow-list rather
/// than a deny-list.
#[test]
#[ignore = "blocked on ENC-090"]
fn tc_api_003_health_endpoint_stays_public() {
    unimplemented!("GET /health must return 200 with no credentials");
}

/// TC-API-004 — error responses must not leak internal detail.
///
/// `dispatch` builds its error body with `format!("{:?}", e)` — the Debug
/// representation of `EnclaveError`. `EnclaveError::Storage` wraps
/// `std::io::Error::to_string()`, which embeds absolute filesystem paths. So
/// a request for a non-existent secret returns the enclave's store layout to
/// an unauthenticated caller.
///
/// `error.rs` opens with: "All error variants are designed to be safe to
/// surface to callers: they MUST NOT embed raw secret material, private key
/// bytes, or plaintext values." The Storage variant violates its own module
/// contract. See issue ENC-094.
#[test]
#[ignore = "dispatch uses format!(\"{:?}\", e) — see issue ENC-094"]
fn tc_api_004_errors_do_not_leak_paths() {
    unimplemented!(
        "replace the Debug formatting with a stable error code + generic \
         message, and log the detail internally only"
    );
}

/// TC-API-005 — request bodies must be read to completion.
///
/// `router::handle_connection` performs a single 64 KiB `read()` and treats
/// whatever arrives as the whole request. TCP makes no such guarantee: any
/// request whose headers and body span more than one segment is silently
/// truncated, and `body_start..` slices into a partial buffer. A large
/// `POST /v1/secrets` stores a truncated secret. See issue ENC-095.
#[test]
#[ignore = "handle_connection does a single read() — see issue ENC-095"]
fn tc_api_005_multi_segment_request_is_read_fully() {
    unimplemented!("loop on read() until Content-Length bytes are buffered");
}

/// TC-API-006 — connection handling must be bounded.
///
/// `start_server` spawns an unbounded OS thread per accepted connection with
/// no read timeout and no concurrency cap. Spec §3 caps the enclave heap at
/// 64 MB (CONFORMANCE_REPORT §3 claims this is "Enforced by SGXS manifest").
/// A few hundred idle connections exhaust EPC and take the enclave down.
/// See issue ENC-096.
#[test]
#[ignore = "unbounded thread-per-connection — see issue ENC-096"]
fn tc_api_006_connection_count_is_bounded() {
    unimplemented!("bound the worker pool and set SO_RCVTIMEO on accepted sockets");
}

/// TC-API-007 — transport must be TLS.
///
/// `start_server` binds a plain `TcpListener` and logs a warning that TLS
/// "should be terminated by a TLS proxy in front of the enclave". Spec §3
/// and §5.1 require mTLS and RA-TLS with DCAP quotes bound into the
/// certificate. `rustls` and `rcgen` are declared in `enclave/Cargo.toml`
/// and imported nowhere. Terminating TLS outside the enclave defeats the
/// entire trust boundary the design is built on: the proxy sees plaintext
/// secrets. See issue ENC-097.
#[test]
#[ignore = "no TLS implementation exists — see issue ENC-097"]
fn tc_api_007_transport_is_ra_tls() {
    unimplemented!("terminate rustls inside the enclave with a DCAP-bound cert");
}
