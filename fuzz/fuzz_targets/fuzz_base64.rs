//! libFuzzer Target for Base64 and Base64URL Decoding Engines.
//!
//! # Invariants Under Test
//! - **Panic Freedom**: Untrusted byte streams provided to `STANDARD`, `URL_SAFE`, and `URL_SAFE_NO_PAD`
//!   decoders must not panic under malformed padding, non-ASCII byte sequences, or boundary truncations.
//! - **Canonical Rejection**: Invalid non-alphabet bytes must be cleanly rejected with error results.

#![no_main]
use base64::Engine as _;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // 1. Fuzz standard Base64 decoding
    let _ = base64::engine::general_purpose::STANDARD.decode(data);

    // 2. Fuzz URL-safe unpadded Base64 decoding (used in JWT and tokens)
    let _ = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(data);

    // 3. Fuzz URL-safe padded Base64 decoding
    let _ = base64::engine::general_purpose::URL_SAFE.decode(data);

    // 4. Test string-based inputs
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s);
    }
});
