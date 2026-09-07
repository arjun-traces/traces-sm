//! Post-Quantum Cryptography (PQC) API module.
//!
//! PQC algorithms (ML-KEM-768/1024 NIST FIPS 203 and ML-DSA-3/5 NIST FIPS 204)
//! are currently stubs and return `EnclaveError::NotImplemented`.

use crate::error::EnclaveError;

pub struct MlKemKeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

pub struct MlDsaKeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

pub fn generate_ml_kem_768_keypair() -> Result<MlKemKeyPair, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-KEM-768 key generation is not implemented".to_string(),
    ))
}

pub fn generate_ml_kem_1024_keypair() -> Result<MlKemKeyPair, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-KEM-1024 key generation is not implemented".to_string(),
    ))
}

pub fn ml_kem_encapsulate(_public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-KEM encapsulation is not implemented".to_string(),
    ))
}

pub fn ml_kem_decapsulate(_secret_key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-KEM decapsulation is not implemented".to_string(),
    ))
}

pub fn generate_ml_dsa_3_keypair() -> Result<MlDsaKeyPair, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-DSA-3 key generation is not implemented".to_string(),
    ))
}

pub fn generate_ml_dsa_5_keypair() -> Result<MlDsaKeyPair, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-DSA-5 key generation is not implemented".to_string(),
    ))
}

pub fn generate_ml_dsa_87_keypair() -> Result<MlDsaKeyPair, EnclaveError> {
    generate_ml_dsa_5_keypair()
}

pub fn ml_dsa_sign(_secret_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-DSA signing is not implemented".to_string(),
    ))
}

pub fn ml_dsa_verify(
    _public_key: &[u8],
    _message: &[u8],
    _signature: &[u8],
) -> Result<bool, EnclaveError> {
    Err(EnclaveError::NotImplemented(
        "ML-DSA verification is not implemented".to_string(),
    ))
}

pub fn ml_dsa_87_verify(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, EnclaveError> {
    ml_dsa_verify(public_key, message, signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pqc_functions_return_not_implemented() {
        assert!(matches!(
            generate_ml_kem_768_keypair(),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            generate_ml_kem_1024_keypair(),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            ml_kem_encapsulate(&[]),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            ml_kem_decapsulate(&[], &[]),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            generate_ml_dsa_3_keypair(),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            generate_ml_dsa_5_keypair(),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            generate_ml_dsa_87_keypair(),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            ml_dsa_sign(&[], &[]),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            ml_dsa_verify(&[], &[], &[]),
            Err(EnclaveError::NotImplemented(_))
        ));
        assert!(matches!(
            ml_dsa_87_verify(&[], &[], &[]),
            Err(EnclaveError::NotImplemented(_))
        ));
    }
}
