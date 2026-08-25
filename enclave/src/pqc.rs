use crate::error::EnclaveError;

/// Post-Quantum Cryptography (PQC) Abstractions (NIST FIPS 203 ML-KEM & FIPS 204 ML-DSA).
///
/// NOTE: Production PQC logic requires hardware/SIM integration with pure-Rust crates (`ml-kem`, `ml-dsa`).
/// Experimental stubs explicitly return `EnclaveError::NotImplemented` until audited algorithms are linked.

pub fn ml_kem_encapsulate(_pub_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), EnclaveError> {
    Err(EnclaveError::NotImplemented("ML-KEM encapsulation is experimental. Use standard RSA/ECDSA/Ed25519 for production.".into()))
}

pub fn ml_kem_decapsulate(_priv_key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>, EnclaveError> {
    Err(EnclaveError::NotImplemented("ML-KEM decapsulation is experimental.".into()))
}

pub fn ml_dsa_sign(_priv_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, EnclaveError> {
    Err(EnclaveError::NotImplemented("ML-DSA signature generation is experimental.".into()))
}

pub fn ml_dsa_verify(_pub_key: &[u8], _message: &[u8], _signature: &[u8]) -> Result<bool, EnclaveError> {
    // Explicitly do NOT return true unconditionally
    Err(EnclaveError::NotImplemented("ML-DSA signature verification is experimental.".into()))
}
