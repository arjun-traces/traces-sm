use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveError {
    #[error("Sealing error: {0}")]
    SealingError(String),

    #[error("Crypto error: {0}")]
    CryptoError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("ZKP error: {0}")]
    ZkpError(String),

    #[error("HE error: {0}")]
    HeError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Key generation failed: {0}")]
    KeyGenFailed(String),

    #[error("RSA encryption failed: {0}")]
    RsaEncryptFailed(String),

    #[error("RSA decryption failed: {0}")]
    RsaDecryptFailed(String),

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),

    #[error("Internal enclave error: {0}")]
    Internal(String),
}
