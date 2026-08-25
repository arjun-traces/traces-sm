//! JWT token service — runs entirely inside the enclave.
//!
//! On first start, generates an Ed25519 signing keypair, seals the private
//! key to disk, and uses it for all subsequent token issuance.  Token JWTs
//! are signed with EdDSA (Ed25519) and verified without leaving the EPC.
//!
//! Token structure:
//!   Header : { "alg": "EdDSA", "typ": "JWT" }
//!   Claims : { sub, iat, exp, jti, scopes }
//!   Sig    : Ed25519 over base64url(header).base64url(payload)

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use base64::Engine as _;
use chrono::Utc;
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::error::EnclaveError;
use crate::sealing::{seal, unseal, SealingKeyProvider};

const SEALING_PURPOSE: &str = "seal:token-signing-key";
const KEY_FILE: &str = "token_signing_key.sealed";

// ─────────────────────────────────────────────────────────────────────────────
// JWT claims
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
    pub scopes: Vec<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Token service
// ─────────────────────────────────────────────────────────────────────────────

pub struct EnclaveTokenService {
    /// Sealed PKCS#8 Ed25519 private key bytes.
    sealed_priv: Vec<u8>,
    /// DER-encoded public key bytes (for verification without unsealing).
    pub_bytes: Vec<u8>,
    /// In-memory deny-list of revoked token JTIs.
    revoked: Arc<Mutex<HashSet<String>>>,
    /// Store path for persisting the sealed key.
    store_path: String,
}

impl EnclaveTokenService {
    /// Load or create the signing keypair.
    pub fn new(store_path: &str, provider: &dyn SealingKeyProvider) -> Result<Self, EnclaveError> {
        let key_path = std::path::Path::new(store_path).join(KEY_FILE);

        let (sealed_priv, pub_bytes) = if key_path.exists() {
            // Load existing sealed key
            let sealed = std::fs::read(&key_path)?;
            let priv_pkcs8 = Zeroizing::new(unseal(&sealed, SEALING_PURPOSE, provider)?);
            let kp = Ed25519KeyPair::from_pkcs8(&priv_pkcs8)
                .map_err(|e| EnclaveError::KeyGen(format!("Ed25519 token key load: {e}")))?;
            let pub_b = kp.public_key().as_ref().to_vec();
            (sealed, pub_b)
        } else {
            // Generate a new keypair and seal it
            let rng = SystemRandom::new();
            let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)
                .map_err(|e| EnclaveError::KeyGen(format!("Ed25519 token keygen: {e}")))?;
            let kp = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())
                .map_err(|e| EnclaveError::KeyGen(format!("Ed25519 token parse: {e}")))?;
            let pub_b = kp.public_key().as_ref().to_vec();
            let sealed = seal(pkcs8.as_ref(), SEALING_PURPOSE, provider)?;
            std::fs::write(&key_path, &sealed)?;
            log::info!("Generated new token signing key at {key_path:?}");
            (sealed, pub_b)
        };

        Ok(Self {
            sealed_priv,
            pub_bytes,
            revoked: Arc::new(Mutex::new(HashSet::new())),
            store_path: store_path.to_string(),
        })
    }

    /// Issue a signed JWT.
    pub fn issue_token(
        &self,
        subject: &str,
        scopes: Vec<String>,
        ttl_secs: u64,
        provider: &dyn SealingKeyProvider,
    ) -> Result<(String /*jti*/, String /*jwt*/), EnclaveError> {
        let now = Utc::now().timestamp();
        let jti = Uuid::new_v4().to_string();
        let claims = Claims {
            sub: subject.to_string(),
            iat: now,
            exp: now + ttl_secs as i64,
            jti: jti.clone(),
            scopes,
        };

        let jwt = self.sign_jwt(&claims, provider)?;
        Ok((jti, jwt))
    }

    /// Verify a JWT and return its claims.
    pub fn verify_token(&self, token: &str) -> Result<Claims, EnclaveError> {
        let parts: Vec<&str> = token.splitn(3, '.').collect();
        if parts.len() != 3 {
            return Err(EnclaveError::Unauthorized);
        }
        let signing_input = format!("{}.{}", parts[0], parts[1]);
        let sig_bytes = b64url_decode(parts[2])?;

        // Verify Ed25519 signature
        let pk = UnparsedPublicKey::new(&ED25519, &self.pub_bytes);
        pk.verify(signing_input.as_bytes(), &sig_bytes)
            .map_err(|_| EnclaveError::Unauthorized)?;

        // Decode claims
        let payload_json = b64url_decode(parts[1])?;
        let claims: Claims = serde_json::from_slice(&payload_json)
            .map_err(|_| EnclaveError::Unauthorized)?;

        // Check expiry
        if Utc::now().timestamp() > claims.exp {
            return Err(EnclaveError::TokenExpired);
        }

        // Check deny-list
        if self.revoked.lock().unwrap().contains(&claims.jti) {
            return Err(EnclaveError::TokenRevoked);
        }

        Ok(claims)
    }

    /// Add a JTI to the in-memory revocation list.
    pub fn revoke_token(&self, jti: &str) {
        self.revoked.lock().unwrap().insert(jti.to_string());
    }

    // ─────────────────────────────────────────────────────────────────────
    // Internal
    // ─────────────────────────────────────────────────────────────────────

    fn sign_jwt(&self, claims: &Claims, provider: &dyn SealingKeyProvider) -> Result<String, EnclaveError> {
        let header = r#"{"alg":"EdDSA","typ":"JWT"}"#;
        let header_b64 = b64url_encode(header.as_bytes());
        let payload_json = serde_json::to_vec(claims)?;
        let payload_b64 = b64url_encode(&payload_json);

        let signing_input = format!("{header_b64}.{payload_b64}");

        // Unseal private key
        let priv_pkcs8 = Zeroizing::new(unseal(&self.sealed_priv, SEALING_PURPOSE, provider)?);
        let kp = Ed25519KeyPair::from_pkcs8(&priv_pkcs8)
            .map_err(|e| EnclaveError::Sign(format!("Ed25519 token sign: {e}")))?;

        let sig = kp.sign(signing_input.as_bytes());
        let sig_b64 = b64url_encode(sig.as_ref());

        Ok(format!("{signing_input}.{sig_b64}"))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Base64url helpers (no padding)
// ─────────────────────────────────────────────────────────────────────────────

fn b64url_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

fn b64url_decode(s: &str) -> Result<Vec<u8>, EnclaveError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| EnclaveError::Unauthorized)
}
