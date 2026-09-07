//! Key generation conformance tests for `enclave/src/keygen.rs`.
//!
//! Spec reference (docs/TECHNICAL_SPECIFICATION.md §4.1) — the declared
//! in-enclave algorithm catalog:
//!
//!   RSA-2048/4096, ECDSA P-256/P-384/P-521/Secp256k1, Ed25519, X25519,
//!   ML-KEM-512/768/1024, ML-DSA-3/5, SLH-DSA-SHA2-128f/256f,
//!   AES-128/256-GCM, AES-KW, HMAC-SHA256/512, ChaCha20, FROST-Ed25519.
//!
//! CONFORMANCE_REPORT.md §2.6 certifies PQC as "✅ 100% CONFORMANT" citing
//! `enclave/src/pqc.rs`. Every function in that file returns
//! `EnclaveError::NotImplemented`. TC-KG-020 is the test that says so.

mod common;

use common::FixedKeyProvider;
use traces_sm_enclave::error::EnclaveError;
use traces_sm_enclave::keygen::generate_key_pair;
use traces_sm_enclave::models::KeyAlgorithm;
use traces_sm_enclave::pqc;

/// Algorithms §4.1 claims are supported for key generation.
const CATALOG: &[KeyAlgorithm] = &[
    KeyAlgorithm::Rsa2048,
    KeyAlgorithm::Rsa4096,
    KeyAlgorithm::EcdsaP256,
    KeyAlgorithm::EcdsaP384,
    KeyAlgorithm::EcdsaP521,
    KeyAlgorithm::Secp256k1,
    KeyAlgorithm::Ed25519,
    KeyAlgorithm::X25519,
    KeyAlgorithm::MlKem512,
    KeyAlgorithm::MlKem768,
    KeyAlgorithm::MlKem1024,
    KeyAlgorithm::MlDsa3,
    KeyAlgorithm::MlDsa5,
    KeyAlgorithm::SlhDsa,
];

// ─────────────────────────────────────────────────────────────────────────────
// The critical one
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KG-001 — CRITICAL: the public key must not contain private key bytes.
///
/// `generate_secp256k1` never touches the secp256k1 curve. It samples 32
/// random bytes as the "private key" and then builds the public key as:
///
///     hex::encode(&priv_bytes[0..16])
///
/// The first 128 bits of the private key are published as the public key,
/// and the remaining 128 bits are all that stand between an attacker and the
/// key — brute-forceable relative to the 256-bit strength the API advertises.
///
/// This is the highest-severity finding in the codebase. See issue ENC-060.
#[test]
fn tc_kg_001_public_key_never_contains_private_material() {
    let p = FixedKeyProvider::new(0x77);

    for alg in CATALOG {
        let Ok(kp) = generate_key_pair(alg.clone(), &p) else {
            continue; // unimplemented algorithms are covered by TC-KG-020
        };

        // Recover the sealed private key so we can look for it in the public half.
        let priv_bytes = traces_sm_enclave::sealing::unseal(
            &kp.sealed_private_key,
            match alg {
                KeyAlgorithm::Secp256k1 => "seal:secp256k1-privkey",
                KeyAlgorithm::Ed25519 => "seal:ed25519-privkey",
                KeyAlgorithm::EcdsaP256 | KeyAlgorithm::EcdsaP384 => "seal:ecdsa-privkey",
                KeyAlgorithm::Rsa2048 | KeyAlgorithm::Rsa4096 => "seal:rsa-privkey",
                _ => "seal:symmetric-key",
            },
            &p,
        )
        .expect("unseal private key");

        // Any 8-byte run of the private key appearing in the public key is a leak.
        let pubkey = kp.public_key_pem.as_bytes();
        let pub_hex = hex::decode(kp.public_key_pem.trim()).unwrap_or_default();

        for window in priv_bytes.windows(8) {
            assert!(
                !pubkey.windows(8).any(|w| w == window),
                "{alg}: 8 bytes of the private key appear verbatim in the \
                 public key PEM (issue ENC-060)"
            );
            assert!(
                !pub_hex.windows(8).any(|w| w == window),
                "{alg}: 8 bytes of the private key appear in the hex-decoded \
                 public key (issue ENC-060)"
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Silent algorithm substitution
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KG-002 — an unimplemented algorithm must error, not substitute.
///
/// `generate_key_pair` ends with a catch-all arm:
///
///     _ => generate_symmetric(32, &algorithm.to_string(), provider)
///
/// So a request for ECDSA P-521, X25519, SLH-DSA, HMAC-SHA512 or
/// ChaCha20-Poly1305 returns 32 random bytes labelled with the requested
/// algorithm name. The caller believes it holds a P-521 signing key. It holds
/// a symmetric blob that cannot sign anything.
///
/// Silent substitution of a weaker or wrong primitive is worse than an
/// outright failure: the failure is invisible until a signature is needed.
/// See issue ENC-061.
#[test]
fn tc_kg_002_unsupported_algorithms_error_rather_than_substitute() {
    let p = FixedKeyProvider::new(0x78);

    // Asymmetric algorithms that hit the catch-all arm today.
    for alg in [
        KeyAlgorithm::EcdsaP521,
        KeyAlgorithm::X25519,
        KeyAlgorithm::SlhDsa,
        KeyAlgorithm::MlKem512,
    ] {
        match generate_key_pair(alg.clone(), &p) {
            Err(EnclaveError::NotImplemented(_)) | Err(EnclaveError::UnsupportedAlgorithm(_)) => {}
            Err(other) => panic!("{alg}: unexpected error {other}"),
            Ok(kp) => panic!(
                "{alg}: silently produced a symmetric key labelled \
                 {:?} instead of failing (issue ENC-061)",
                kp.public_key_pem
            ),
        }
    }
}

/// TC-KG-003 — symmetric algorithms must not advertise a public key.
///
/// `generate_symmetric` returns the string `SYMMETRIC_KEY_<alg>_LENGTH_32B`
/// in the `public_key_pem` field. A field typed as a PEM public key that
/// carries a placeholder string will be served from
/// `GET /v1/keys/{id}/public` to any client.
#[test]
fn tc_kg_003_symmetric_keys_have_no_public_half() {
    let p = FixedKeyProvider::new(0x79);
    let kp = generate_key_pair(KeyAlgorithm::Aes256Gcm, &p).expect("aes keygen");
    assert!(
        !kp.public_key_pem.starts_with("SYMMETRIC_KEY_"),
        "a symmetric key exposed the placeholder {:?} through the \
         public_key_pem field (issue ENC-062)",
        kp.public_key_pem
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// PEM validity
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KG-004 — exported public keys must be valid SPKI PEM.
///
/// ECDSA and Ed25519 keys are emitted as
/// `-----BEGIN PUBLIC KEY-----\n<base64 of raw point>\n-----END PUBLIC KEY-----`.
/// That is not SPKI: the raw point is not wrapped in a
/// SubjectPublicKeyInfo structure, and the base64 is not wrapped at 64
/// columns as RFC 7468 requires. No standard tool (OpenSSL, Go, Java) can
/// parse these. See issue ENC-063.
#[test]
fn tc_kg_004_exported_pem_is_rfc7468_conformant() {
    let p = FixedKeyProvider::new(0x7A);

    for alg in [KeyAlgorithm::Ed25519, KeyAlgorithm::EcdsaP256, KeyAlgorithm::Rsa2048] {
        let kp = generate_key_pair(alg.clone(), &p).expect("keygen");
        let pem = kp.public_key_pem;

        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----"), "{alg}: bad PEM header");
        assert!(pem.trim_end().ends_with("-----END PUBLIC KEY-----"), "{alg}: bad PEM footer");

        let body: String = pem
            .lines()
            .filter(|l| !l.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("\n");

        for line in body.lines() {
            assert!(
                line.len() <= 64,
                "{alg}: PEM body line is {} chars; RFC 7468 §2 requires \
                 wrapping at 64 (issue ENC-063)",
                line.len()
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Post-quantum cryptography
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KG-020 — PQC must be implemented, or the conformance claim withdrawn.
///
/// CONFORMANCE_REPORT.md §1 row 7 and §2.6 both certify Post-Quantum
/// Cryptography as "✅ 100% CONFORMANT" with `enclave/src/pqc.rs` as the
/// evidence. Every function in that file is a stub returning
/// `NotImplemented` — and the module's own unit test *asserts* that they do,
/// pinning the non-conformance as expected behaviour.
///
/// §9's threat matrix lists "Quantum Computing Decryption" as mitigated by
/// "ML-KEM-768 & ML-DSA-3 PQC". That mitigation does not exist.
///
/// This test fails until FIPS 203/204 are actually implemented (the `ml-kem`
/// and `ml-dsa` crates, or `pqcrypto`). See issue ENC-064.
#[test]
fn tc_kg_020_pqc_is_implemented() {
    let stubs: Vec<(&str, bool)> = vec![
        ("ML-KEM-768 keygen", pqc::generate_ml_kem_768_keypair().is_err()),
        ("ML-KEM-1024 keygen", pqc::generate_ml_kem_1024_keypair().is_err()),
        ("ML-KEM encapsulate", pqc::ml_kem_encapsulate(&[]).is_err()),
        ("ML-DSA-3 keygen", pqc::generate_ml_dsa_3_keypair().is_err()),
        ("ML-DSA-5 keygen", pqc::generate_ml_dsa_5_keypair().is_err()),
        ("ML-DSA sign", pqc::ml_dsa_sign(&[], b"m").is_err()),
    ];

    let missing: Vec<&str> = stubs.iter().filter(|(_, e)| *e).map(|(n, _)| *n).collect();
    assert!(
        missing.is_empty(),
        "{} of {} PQC primitives are unimplemented stubs while \
         CONFORMANCE_REPORT.md §2.6 certifies them 100% conformant: {:?} \
         (issue ENC-064)",
        missing.len(),
        stubs.len(),
        missing
    );
}

/// TC-KG-021 — SLH-DSA (FIPS 205) is in §4.1 but has no module at all.
#[test]
fn tc_kg_021_slh_dsa_exists() {
    let p = FixedKeyProvider::new(0x7B);
    assert!(
        matches!(
            generate_key_pair(KeyAlgorithm::SlhDsa, &p),
            Err(EnclaveError::NotImplemented(_))
        ),
        "SLH-DSA-SHA2-128f/256f is listed in spec §4.1 with no implementation \
         and no stub — it currently falls through to the symmetric catch-all \
         (issues ENC-061, ENC-064)"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// AES key wrap (SP 800-38F)
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KG-030 — AES-KW must exist.
///
/// CONFORMANCE_REPORT.md §1 row 5 certifies "NIST SP 800-38F Key Wrap"
/// as conformant, citing `enclave/src/crypto.rs`. That file contains
/// `encrypt_secret` and `decrypt_secret` only — envelope encryption with
/// AES-GCM. There is no AES-KW or AES-KWP implementation anywhere in the
/// crate. `KeyAlgorithm::Aes128Kw` / `Aes256Kw` generate raw random bytes and
/// no code ever wraps anything with them. See issue ENC-065.
#[test]
#[ignore = "no AES-KW implementation exists anywhere in the crate — issue ENC-065"]
fn tc_kg_030_aes_kw_wraps_and_unwraps() {
    unimplemented!(
        "implement RFC 3394 AES-KW / RFC 5649 AES-KWP and test against the \
         RFC 3394 §4 test vectors"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Signing
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KG-040 — sign/verify must round-trip for every signing algorithm.
///
/// docs/TASK_CHECKLIST.md Phase 2 marks `keygen.rs` complete with
/// "RSA-4096, ECDSA P-256/P-384/Secp256k1, Ed25519 gen+sign+verify". No
/// `sign` or `verify_signature` function exists in `keygen.rs`.
/// `handlers/keys.rs` calls `keygen::sign(...)`, `keygen::verify_signature(...)`,
/// `keygen::rsa_encrypt(...)` and `keygen::rsa_decrypt(...)` — none of which
/// are defined, which is one of the reasons the crate does not compile.
/// See issues ENC-066 and ENC-070.
#[test]
#[ignore = "keygen::sign / verify_signature do not exist — see issue ENC-066"]
fn tc_kg_040_sign_verify_roundtrip() {
    unimplemented!("implement keygen::sign and keygen::verify_signature");
}
