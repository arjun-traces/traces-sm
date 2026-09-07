#![no_main]
use libfuzzer_sys::fuzz_target;
use base64::Engine as _;

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
