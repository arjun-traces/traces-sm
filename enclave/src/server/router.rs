//! HTTP request router — parses raw HTTP/1.1 and dispatches to handlers.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use crate::error::{EnclaveError, http_status};
use crate::models::ApiResponse;
use crate::server::handlers;
use crate::server::EnclaveState;

// ─────────────────────────────────────────────────────────────────────────────
// Minimal HTTP request / response structures
// ─────────────────────────────────────────────────────────────────────────────

pub struct HttpRequest {
    pub method:  String,
    pub path:    String,
    pub headers: Vec<(String, String)>,
    pub body:    Vec<u8>,
}

impl HttpRequest {
    pub fn header(&self, name: &str) -> Option<&str> {
        let name_lower = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == name_lower)
            .map(|(_, v)| v.as_str())
    }

    pub fn bearer_token(&self) -> Option<&str> {
        self.header("Authorization")
            .and_then(|v| v.strip_prefix("Bearer "))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Connection handler
// ─────────────────────────────────────────────────────────────────────────────

pub fn handle_connection(
    mut stream: TcpStream,
    state: Arc<EnclaveState>,
) -> Result<(), EnclaveError> {
    // Read up to 64 KiB
    let mut buf = vec![0u8; 65536];
    let n = stream.read(&mut buf).map_err(|e| EnclaveError::Storage(e.to_string()))?;
    buf.truncate(n);

    let req = parse_request(&buf)?;
    let response = dispatch(&req, state);
    stream.write_all(response.as_bytes())
        .map_err(|e| EnclaveError::Storage(e.to_string()))?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Request parsing (minimal HTTP/1.1)
// ─────────────────────────────────────────────────────────────────────────────

fn parse_request(raw: &[u8]) -> Result<HttpRequest, EnclaveError> {
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);

    let status = req.parse(raw)
        .map_err(|e| EnclaveError::BadRequest(format!("HTTP parse error: {e}")))?;

    let method = req.method.unwrap_or("GET").to_string();
    let path   = req.path.unwrap_or("/").to_string();

    let header_list: Vec<(String, String)> = req.headers.iter()
        .filter(|h| !h.name.is_empty())
        .map(|h| (h.name.to_string(), String::from_utf8_lossy(h.value).to_string()))
        .collect();

    let body_start = match status {
        httparse::Status::Complete(n) => n,
        httparse::Status::Partial => {
            return Err(EnclaveError::BadRequest("incomplete HTTP request".into()));
        }
    };

    let body = raw[body_start..].to_vec();

    Ok(HttpRequest { method, path, headers: header_list, body })
}

// ─────────────────────────────────────────────────────────────────────────────
// Dispatcher
// ─────────────────────────────────────────────────────────────────────────────

fn dispatch(req: &HttpRequest, state: Arc<EnclaveState>) -> String {
    // Segment the path: "/v1/secrets/some-id" → ["v1", "secrets", "some-id"]
    let segments: Vec<&str> = req.path
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let result: Result<serde_json::Value, EnclaveError> = match (req.method.as_str(), segments.as_slice()) {
        // ── Health ────────────────────────────────────────────────────────────
        ("GET", ["health"]) => {
            Ok(serde_json::json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }))
        }

        // ── Secrets ───────────────────────────────────────────────────────────
        ("POST",   ["v1", "secrets"])     => handlers::secrets::create(req, &state),
        ("GET",    ["v1", "secrets"])     => handlers::secrets::list(req, &state),
        ("GET",    ["v1", "secrets", id]) => handlers::secrets::get(req, &state, id),
        ("PUT",    ["v1", "secrets", id]) => handlers::secrets::update(req, &state, id),
        ("DELETE", ["v1", "secrets", id]) => handlers::secrets::delete(req, &state, id),

        // ── Keys ──────────────────────────────────────────────────────────────
        ("POST",   ["v1", "keys"])                  => handlers::keys::generate(req, &state),
        ("GET",    ["v1", "keys"])                  => handlers::keys::list(req, &state),
        ("GET",    ["v1", "keys", id, "public"])    => handlers::keys::public_key(req, &state, id),
        ("POST",   ["v1", "keys", id, "sign"])      => handlers::keys::sign(req, &state, id),
        ("POST",   ["v1", "keys", id, "verify"])    => handlers::keys::verify(req, &state, id),
        ("POST",   ["v1", "keys", id, "encrypt"])   => handlers::keys::encrypt(req, &state, id),
        ("POST",   ["v1", "keys", id, "decrypt"])   => handlers::keys::decrypt(req, &state, id),
        ("POST",   ["v1", "keys", id, "rotate"])    => handlers::keys::rotate(req, &state, id),
        ("DELETE", ["v1", "keys", id])              => handlers::keys::delete(req, &state, id),

        // ── Tokens ────────────────────────────────────────────────────────────
        ("POST",   ["v1", "tokens"])     => handlers::tokens::create(req, &state),
        ("GET",    ["v1", "tokens"])     => handlers::tokens::list(req, &state),
        ("DELETE", ["v1", "tokens", id]) => handlers::tokens::revoke(req, &state, id),

        // ── ZKP ───────────────────────────────────────────────────────────────
        ("POST",   ["v1", "zkp", "schnorr", "prove"])        => handlers::zkp::schnorr_prove(req, &state),
        ("POST",   ["v1", "zkp", "schnorr", "verify"])       => handlers::zkp::schnorr_verify(req, &state),
        ("POST",   ["v1", "zkp", "range", "prove"])          => handlers::zkp::range_prove(req, &state),
        ("POST",   ["v1", "zkp", "range", "verify"])         => handlers::zkp::range_verify(req, &state),
        ("POST",   ["v1", "zkp", "pedersen", "commit"])      => handlers::zkp::pedersen_commit(req, &state),
        ("POST",   ["v1", "zkp", "he", "generate"])          => handlers::zkp::he_generate(req, &state),
        ("POST",   ["v1", "zkp", "he", "encrypt"])           => handlers::zkp::he_encrypt(req, &state),
        ("POST",   ["v1", "zkp", "he", "add"])               => handlers::zkp::he_add(req, &state),
        ("POST",   ["v1", "zkp", "he", "decrypt"])           => handlers::zkp::he_decrypt(req, &state),        // ── Entropy (NIST SP 800-90B) ──────────────────────────────────────────
        ("GET",  ["v1", "entropy", "health"])     => {
            let status = crate::drbg::init_drbg_health_check();
            Ok(serde_json::json!({ "rct_passed": status.rct_passed, "apt_passed": status.apt_passed, "reseed_count": status.reseed_count, "source": "SGX_RDRAND_RDSEED" }))
        },



        // ── Attestation ───────────────────────────────────────────────────────
        ("GET",  ["v1", "attest", "quote"])        => handlers::attest::quote(req, &state),
        ("GET",  ["v1", "attest", "measurements"]) => handlers::attest::measurements(req, &state),
        ("POST", ["v1", "attest", "verify"])       => handlers::attest::verify(req, &state),

        // ── 404 ───────────────────────────────────────────────────────────────
        _ => Err(EnclaveError::NotFound { id: req.path.clone() }),
    };

    match result {
        Ok(body) => http_response(200, &serde_json::json!({ "success": true, "data": body })),
        Err(e) => {
            let status = http_status(&e);
            http_response(status, &ApiResponse::<()>::err(format!("{:?}", e), e.to_string()))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Response formatting
// ─────────────────────────────────────────────────────────────────────────────

fn http_response(status: u16, body: &impl serde::Serialize) -> String {
    let json = serde_json::to_string_pretty(body).unwrap_or_else(|_| r#"{"error":"serialization failed"}"#.to_string());
    let reason = match status {
        200 => "OK", 400 => "Bad Request", 401 => "Unauthorized",
        403 => "Forbidden", 404 => "Not Found", 409 => "Conflict",
        _   => "Internal Server Error",
    };
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nX-Enclave: traces-sm-enclave-v{}\r\n\r\n{}",
        json.len(),
        env!("CARGO_PKG_VERSION"),
        json
    )
}


