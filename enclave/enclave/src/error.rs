//! Enclave-wide error types.
//!
//! All error variants are designed to be safe to surface to callers:
//! they MUST NOT embed raw secret material, private key bytes, or
//! plaintext values in their messages.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnclaveError {
    // ------------------------------------------------------------------
    // Sealing / unsealing
    // ------------------------------------------------------------------
    #[error("sealing error: {msg}")]
    Sealing { msg: &'static str },

    #[error("unsealing error: {msg}")]
    Unsealing { msg: &'static str },

    // ------------------------------------------------------------------
    // Symmetric crypto
    // ------------------------------------------------------------------
    #[error("AES-GCM encryption failed")]
    AesGcmEncrypt,

    #[error("AES-GCM decryption failed (authentication tag mismatch)")]
    AesGcmDecrypt,

    #[error("HKDF key derivation failed: {0}")]
    Hkdf(String),

    // ------------------------------------------------------------------
    // Asymmetric key operations
    // ------------------------------------------------------------------
    #[error("key generation failed: {0}")]
    KeyGen(String),

    #[error("signing failed: {0}")]
    Sign(String),

    #[error("signature verification failed")]
    Verify,

    #[error("RSA encryption failed: {0}")]
    RsaEncrypt(String),

    #[error("RSA decryption failed: {0}")]
    RsaDecrypt(String),

    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    // ------------------------------------------------------------------
    // ZKP
    // ------------------------------------------------------------------
    #[error("ZKP proof generation failed: {0}")]
    ZkpProve(String),

    #[error("ZKP proof verification failed")]
    ZkpVerify,

    #[error("ZKP invalid input: {0}")]
    ZkpInvalidInput(String),

    // ------------------------------------------------------------------
    // Homomorphic Encryption
    // ------------------------------------------------------------------
    #[error("HE key generation failed: {0}")]
    HeKeyGen(String),

    #[error("HE encryption failed: {0}")]
    HeEncrypt(String),

    #[error("HE decryption failed: {0}")]
    HeDecrypt(String),

    #[error("HE operation failed: {0}")]
    HeOperation(String),

    // ------------------------------------------------------------------
    // Storage
    // ------------------------------------------------------------------
    #[error("storage I/O error: {0}")]
    Storage(String),

    #[error("secret not found: {id}")]
    NotFound { id: String },

    #[error("secret already exists: {id}")]
    AlreadyExists { id: String },

    // ------------------------------------------------------------------
    // Authentication / authorization
    // ------------------------------------------------------------------
    #[error("unauthorized")]
    Unauthorized,

    #[error("token expired")]
    TokenExpired,

    #[error("token revoked")]
    TokenRevoked,

    #[error("insufficient scope: required '{required}'")]
    InsufficientScope { required: String },

    // ------------------------------------------------------------------
    // HTTP / protocol
    // ------------------------------------------------------------------
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("JSON serialization error: {0}")]
    Json(String),

    #[error("TLS error: {0}")]
    Tls(String),

    // ------------------------------------------------------------------
    // Internal / catchall
    // ------------------------------------------------------------------
    #[error("internal error")]
    Internal,
}

impl From<serde_json::Error> for EnclaveError {
    fn from(e: serde_json::Error) -> Self {
        EnclaveError::Json(e.to_string())
    }
}

impl From<std::io::Error> for EnclaveError {
    fn from(e: std::io::Error) -> Self {
        EnclaveError::Storage(e.to_string())
    }
}

/// HTTP status code for a given error.
pub fn http_status(e: &EnclaveError) -> u16 {
    match e {
        EnclaveError::NotFound { .. } => 404,
        EnclaveError::Unauthorized
        | EnclaveError::TokenExpired
        | EnclaveError::TokenRevoked => 401,
        EnclaveError::InsufficientScope { .. } => 403,
        EnclaveError::BadRequest(_) => 400,
        EnclaveError::AlreadyExists { .. } => 409,
        _ => 500,
    }
}
