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
