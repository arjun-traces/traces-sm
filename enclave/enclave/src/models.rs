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
