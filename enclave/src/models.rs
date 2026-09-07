use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SecretType {
    Opaque,
    SymmetricKey,
    AsymmetricKey,
    CertBundle,
    SshKeyPair,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum KeyAlgorithm {
    // Classic Asymmetric
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    EcdsaP521,
    Secp256k1,
    Ed25519,
    X25519,
    // Post-Quantum (NIST FIPS 203/204/205)
    MlKem512,
    MlKem768,
    MlKem1024,
    MlDsa3,
    MlDsa5,
    SlhDsa,
    // Symmetric & Key Wrap (SP 800-38F)
    Aes128Gcm,
    Aes256Gcm,
    Aes128Kw,
    Aes256Kw,
    HmacSha256,
    HmacSha512,
    ChaCha20Poly1305,
    // Threshold DKG
    FrostEd25519,
    PedersenVss,
}

impl std::fmt::Display for KeyAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyAlgorithm::Rsa2048 => write!(f, "RSA-2048"),
            KeyAlgorithm::Rsa4096 => write!(f, "RSA-4096"),
            KeyAlgorithm::EcdsaP256 => write!(f, "ECDSA-P256"),
            KeyAlgorithm::EcdsaP384 => write!(f, "ECDSA-P384"),
            KeyAlgorithm::EcdsaP521 => write!(f, "ECDSA-P521"),
            KeyAlgorithm::Secp256k1 => write!(f, "Secp256k1"),
            KeyAlgorithm::Ed25519 => write!(f, "Ed25519"),
            KeyAlgorithm::X25519 => write!(f, "X25519"),
            KeyAlgorithm::MlKem512 => write!(f, "ML-KEM-512"),
            KeyAlgorithm::MlKem768 => write!(f, "ML-KEM-768"),
            KeyAlgorithm::MlKem1024 => write!(f, "ML-KEM-1024"),
            KeyAlgorithm::MlDsa3 => write!(f, "ML-DSA-3"),
            KeyAlgorithm::MlDsa5 => write!(f, "ML-DSA-5"),
            KeyAlgorithm::SlhDsa => write!(f, "SLH-DSA"),
            KeyAlgorithm::Aes128Gcm => write!(f, "AES-128-GCM"),
            KeyAlgorithm::Aes256Gcm => write!(f, "AES-256-GCM"),
            KeyAlgorithm::Aes128Kw => write!(f, "AES-128-KW"),
            KeyAlgorithm::Aes256Kw => write!(f, "AES-256-KW"),
            KeyAlgorithm::HmacSha256 => write!(f, "HMAC-SHA256"),
            KeyAlgorithm::HmacSha512 => write!(f, "HMAC-SHA512"),
            KeyAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            KeyAlgorithm::FrostEd25519 => write!(f, "FROST-Ed25519"),
            KeyAlgorithm::PedersenVss => write!(f, "Pedersen-VSS"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DkgSetupRequest {
    pub secret_id: uuid::Uuid,
    pub threshold: usize,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrostDkgSetupRequest {
    pub max_signers: u16,
    pub min_signers: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrostCommitRequest {
    pub key_package_json: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrostSignShareRequest {
    pub key_package_json: String,
    pub nonces_json: String,
    pub commitments_map_json: std::collections::BTreeMap<String, String>,
    pub message_hex: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrostAggregateRequest {
    pub public_key_package_json: String,
    pub commitments_map_json: std::collections::BTreeMap<String, String>,
    pub signature_shares_json: std::collections::BTreeMap<String, String>,
    pub message_hex: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrostVerifyRequest {
    pub group_public_key_hex: String,
    pub signature_hex: String,
    pub message_hex: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransitionStateRequest {
    pub new_state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoShredRequest {
    pub id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntropyStatusResponse {
    pub rct_passed: bool,
    pub apt_passed: bool,
    pub reseed_count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(reason: impl Into<String>, _msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(reason.into()),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Handler DTOs
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSecretRequest {
    pub name: String,
    pub secret_type: Option<SecretType>,
    pub plaintext_base64: String,
    pub owner: Option<String>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSecretRequest {
    pub plaintext_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub plaintext_base64: Option<String>,
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretMetadata {
    pub id: uuid::Uuid,
    pub name: String,
    pub secret_type: SecretType,
    pub version: u32,
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateKeyRequest {
    pub name: String,
    pub algorithm: KeyAlgorithm,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub public_key_pem: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRequest {
    pub data_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignResponse {
    pub signature_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyRequest {
    pub data_base64: String,
    pub signature_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyResponse {
    pub valid: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptRequest {
    pub plaintext_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptResponse {
    pub ciphertext_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptRequest {
    pub ciphertext_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptResponse {
    pub plaintext_base64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTokenRequest {
    pub subject: String,
    pub scopes: Vec<String>,
    pub ttl_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttestationMeasurements {
    pub mr_enclave: String,
    pub mr_signer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttestationQuoteResponse {
    pub quote_hex: String,
    pub measurements: AttestationMeasurements,
}


