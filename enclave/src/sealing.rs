//! SGX sealing abstraction.
//!
//! Sealing = encrypt-then-store using a hardware-derived (or simulation) key.
//!
//! Layout of a sealed blob:
//!   [0..12]   — AES-GCM nonce (96-bit)
//!   [12..end] — AES-GCM ciphertext || authentication tag (16 bytes at end)
//!
//! Key hierarchy:
//!   master_key  (from EGETKEY or sim file)
//!        │
//!   HKDF-SHA256(salt="", info=purpose_label)
//!        │
//!   32-byte DEK  →  AES-256-GCM

use std::fs;
use std::path::Path;

use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey,
                 UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::hkdf;
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::Zeroizing;

use crate::error::EnclaveError;

// ─────────────────────────────────────────────────────────────────────────────
// Trait
// ─────────────────────────────────────────────────────────────────────────────

/// Abstraction over the SGX sealing key source.
/// Implementations must be `Send + Sync` so the state can be shared across
/// the server thread pool.
pub trait SealingKeyProvider: Send + Sync {
    /// Return the 32-byte master sealing key.
    ///
    /// On real hardware this calls EGETKEY with `KEYPOLICY_MRSIGNER`.
    /// In simulation mode it reads / creates a file-backed random key.
    fn master_key(&self) -> Result<Zeroizing<[u8; 32]>, EnclaveError>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Simulation provider (dev / CI — no real SGX)
// ─────────────────────────────────────────────────────────────────────────────

pub struct SimSealingProvider {
    key_path: std::path::PathBuf,
}

impl SimSealingProvider {
    pub fn new(store_path: &str) -> Self {
        fs::create_dir_all(store_path)
            .expect("Cannot create store directory");
        Self { key_path: Path::new(store_path).join(".sim_master_key") }
    }
}

impl SealingKeyProvider for SimSealingProvider {
    fn master_key(&self) -> Result<Zeroizing<[u8; 32]>, EnclaveError> {
        if self.key_path.exists() {
            let bytes = fs::read(&self.key_path)
                .map_err(|e| EnclaveError::Sealing { msg: "cannot read sim key" })?;
            if bytes.len() < 32 {
                return Err(EnclaveError::Sealing { msg: "sim key file too short" });
            }
            let mut key = Zeroizing::new([0u8; 32]);
            key.copy_from_slice(&bytes[..32]);
            Ok(key)
        } else {
            let rng = SystemRandom::new();
            let mut raw = Zeroizing::new([0u8; 32]);
            rng.fill(raw.as_mut())
                .map_err(|_| EnclaveError::Sealing { msg: "RNG failure during key gen" })?;
            fs::write(&self.key_path, raw.as_ref())
                .map_err(|_| EnclaveError::Sealing { msg: "cannot write sim key" })?;
            log::info!("Simulation master sealing key created at {:?}", self.key_path);
            Ok(raw)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hardware provider (real Intel SGX — requires sgx-hw feature)
// ─────────────────────────────────────────────────────────────────────────────

pub struct HwSealingProvider;

impl SealingKeyProvider for HwSealingProvider {
    fn master_key(&self) -> Result<Zeroizing<[u8; 32]>, EnclaveError> {
        #[cfg(feature = "sgx-hw")]
        {
            use sgx_isa::{Keyname, Keypolicy, Keyrequest};

            let mut req = Keyrequest::default();
            req.keyname = Keyname::Seal as u16;
            // MRSIGNER policy: key survives code updates, locked to signing identity + SVN
            req.keypolicy = Keypolicy::MRSIGNER;
            req.isvsvn = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap_or(1);

            let raw16 = req.egetkey()
                .map_err(|_| EnclaveError::Sealing { msg: "EGETKEY failed" })?;

            // EGETKEY returns 16 bytes; expand to 32 via SHA-256
            let expanded = ring::digest::digest(&ring::digest::SHA256, &raw16);
            let mut key = Zeroizing::new([0u8; 32]);
            key.copy_from_slice(expanded.as_ref());
            Ok(key)
        }
        #[cfg(not(feature = "sgx-hw"))]
        {
            Err(EnclaveError::Sealing { msg: "SGX hardware feature not compiled in" })
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Nonce wrapper
// ─────────────────────────────────────────────────────────────────────────────

/// Single-use nonce wrapper for ring's `BoundKey` API.
struct OneTimeNonce(Option<[u8; NONCE_LEN]>);

impl NonceSequence for OneTimeNonce {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        self.0.take()
            .map(|b| Nonce::assume_unique_for_key(b))
            .ok_or(ring::error::Unspecified)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Key derivation
// ─────────────────────────────────────────────────────────────────────────────

/// Derive a 32-byte purpose-scoped DEK from the master sealing key using
/// HKDF-SHA256.
///
/// `purpose` MUST be a stable, unique ASCII string per use-case
/// (e.g. `"seal:secrets"`, `"seal:paillier-priv"`, `"seal:token-key"`).
fn derive_dek(
    master: &[u8; 32],
    purpose: &str,
) -> Result<Zeroizing<[u8; 32]>, EnclaveError> {
    // HKDF: Extract → expand
    let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"traces-sm-enclave-v1");
    let prk = salt.extract(master.as_ref());
    let okm = prk
        .expand(&[purpose.as_bytes()], &AES_256_GCM)
        .map_err(|_| EnclaveError::Hkdf(format!("expand failed for purpose={purpose}")))?;

    let mut dek = Zeroizing::new([0u8; 32]);
    okm.fill(dek.as_mut())
        .map_err(|_| EnclaveError::Hkdf("fill failed".into()))?;
    Ok(dek)
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Seal `plaintext` under a purpose-scoped DEK derived from the master key.
///
/// Returns `[nonce(12) | ciphertext | tag(16)]`.
pub fn seal(
    plaintext: &[u8],
    purpose: &str,
    provider: &dyn SealingKeyProvider,
) -> Result<Vec<u8>, EnclaveError> {
    let master = provider.master_key()?;
    let dek = derive_dek(&master, purpose)?;

    // Generate random nonce
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| EnclaveError::AesGcmEncrypt)?;

    let unbound = UnboundKey::new(&AES_256_GCM, dek.as_ref())
        .map_err(|_| EnclaveError::AesGcmEncrypt)?;
    let mut sealing_key = SealingKey::new(unbound, OneTimeNonce(Some(nonce_bytes)));

    let mut in_out = plaintext.to_vec();
    sealing_key
        .seal_in_place_append_tag(Aad::empty(), &mut in_out)
        .map_err(|_| EnclaveError::AesGcmEncrypt)?;

    // Prepend nonce to output
    let mut blob = Vec::with_capacity(NONCE_LEN + in_out.len());
    blob.extend_from_slice(&nonce_bytes);
    blob.extend(in_out);
    Ok(blob)
}

/// Unseal a blob previously produced by `seal()`.
pub fn unseal(
    blob: &[u8],
    purpose: &str,
    provider: &dyn SealingKeyProvider,
) -> Result<Vec<u8>, EnclaveError> {
    // Minimum: 12-byte nonce + 16-byte tag (empty plaintext would be 28 bytes)
    if blob.len() < NONCE_LEN + 16 {
        return Err(EnclaveError::Unsealing { msg: "blob too short" });
    }

    let master = provider.master_key()?;
    let dek = derive_dek(&master, purpose)?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    nonce_bytes.copy_from_slice(&blob[..NONCE_LEN]);

    let unbound = UnboundKey::new(&AES_256_GCM, dek.as_ref())
        .map_err(|_| EnclaveError::AesGcmDecrypt)?;
    let mut opening_key = OpeningKey::new(unbound, OneTimeNonce(Some(nonce_bytes)));

    let mut in_out = blob[NONCE_LEN..].to_vec();
    let decrypted = opening_key
        .open_in_place(Aad::empty(), &mut in_out)
        .map_err(|_| EnclaveError::AesGcmDecrypt)?;

    Ok(decrypted.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockProvider([u8; 32]);
    impl SealingKeyProvider for MockProvider {
        fn master_key(&self) -> Result<Zeroizing<[u8; 32]>, EnclaveError> {
            Ok(Zeroizing::new(self.0))
        }
    }

    #[test]
    fn seal_unseal_roundtrip() {
        let p = MockProvider([0xAB; 32]);
        let msg = b"top secret value 42!";
        let blob = seal(msg, "test:roundtrip", &p).unwrap();
        let recovered = unseal(&blob, "test:roundtrip", &p).unwrap();
        assert_eq!(&recovered, msg);
    }

    #[test]
    fn wrong_purpose_fails() {
        let p = MockProvider([0x12; 32]);
        let blob = seal(b"data", "purpose:A", &p).unwrap();
        // Different purpose → different DEK → tag mismatch
        assert!(unseal(&blob, "purpose:B", &p).is_err());
    }
}

