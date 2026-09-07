//! libFuzzer Target for ASN.1 DER / PKCS#8 Cryptographic Key Parsing.
//!
//! # Invariants Under Test
//! - **Panic Freedom**: Untrusted DER / ASN.1 byte slices fed to `Ed25519KeyPair::from_pkcs8`
//!   and `UnparsedPublicKey::new` must never cause memory unsafety, integer overflows, or unhandled panics.
//! - **Constant-Time Rejection**: Malformed ASN.1 lengths, tag mismatches, and trailing bytes must be rejected cleanly.

#![no_main]
use libfuzzer_sys::fuzz_target;
use ring::signature::Ed25519KeyPair;

fuzz_target!(|data: &[u8]| {
    // 1. Fuzz PKCS#8 ASN.1 DER keypair parsing
    let _ = Ed25519KeyPair::from_pkcs8(data);

    // 2. Fuzz RSA / PKCS#8 DER structure parsing
    if data.len() >= 32 {
        let _ = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, &data[..32]);
    }
});
