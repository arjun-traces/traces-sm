//! File-based sealed blob store.
//!
//! Each secret is stored as two files in `store_path/`:
//!   {id}.meta.json  — SecretMetadata (NOT sensitive; plaintext JSON)
//!   {id}.blob       — EncryptedSecret (AES-256-GCM sealed; opaque bytes)
//!
//! The store is append-friendly and survives restarts.
//! Soft-deleted secrets have `deleted_at` set in metadata; their blob files
//! are retained until GC.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::EnclaveError;
use crate::models::SecretType;

// ─────────────────────────────────────────────────────────────────────────────
// Persisted metadata (stored in plaintext — no secret values here)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretRecord {
    pub id: Uuid,
    pub name: String,
    pub secret_type: SecretType,
    pub version: u32,
    /// For asymmetric keys: PEM-encoded public key.
    pub public_key_pem: Option<String>,
    /// Algorithm label (e.g. "RSA-4096", "Ed25519").
    pub algorithm: Option<String>,
    pub owner: String,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// Schnorr commitment (hex-encoded public key) for ZKP.
    pub zkp_commitment: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Store
// ─────────────────────────────────────────────────────────────────────────────

pub struct Store {
    base: PathBuf,
}

impl Store {
    pub fn new(store_path: &str) -> Self {
        let base = PathBuf::from(store_path);
        fs::create_dir_all(&base).expect("Cannot create store directory");
        Self { base }
    }

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn meta_path(&self, id: &Uuid) -> PathBuf {
        self.base.join(format!("{id}.meta.json"))
    }

    fn blob_path(&self, id: &Uuid) -> PathBuf {
        self.base.join(format!("{id}.blob"))
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    /// Persist a new or updated secret record + its sealed blob.
    pub fn save(&self, record: &SecretRecord, blob: &[u8]) -> Result<(), EnclaveError> {
        let meta_json = serde_json::to_vec_pretty(record)?;
        let m_path = self.meta_path(&record.id);
        let b_path = self.blob_path(&record.id);
        fs::write(&m_path, &meta_json)?;
        fs::write(&b_path, blob)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&m_path, fs::Permissions::from_mode(0o600));
            let _ = fs::set_permissions(&b_path, fs::Permissions::from_mode(0o600));
        }
        Ok(())
    }

    /// Update only the metadata (e.g. after soft-delete or rotation).
    pub fn save_meta(&self, record: &SecretRecord) -> Result<(), EnclaveError> {
        let meta_json = serde_json::to_vec_pretty(record)?;
        let m_path = self.meta_path(&record.id);
        fs::write(&m_path, &meta_json)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&m_path, fs::Permissions::from_mode(0o600));
        }
        Ok(())
    }

    // ── Read ──────────────────────────────────────────────────────────────────

    /// Load a secret record by ID.
    pub fn load(&self, id: &Uuid) -> Result<(SecretRecord, Vec<u8>), EnclaveError> {
        let meta_path = self.meta_path(id);
        if !meta_path.exists() {
            return Err(EnclaveError::NotFound { id: id.to_string() });
        }
        let meta_bytes = fs::read(&meta_path)?;
        let record: SecretRecord = serde_json::from_slice(&meta_bytes)?;

        if record.deleted_at.is_some() {
            return Err(EnclaveError::NotFound { id: id.to_string() });
        }

        let blob = fs::read(self.blob_path(id))?;
        Ok((record, blob))
    }

    /// Load only the metadata (no blob I/O).
    pub fn load_meta(&self, id: &Uuid) -> Result<SecretRecord, EnclaveError> {
        let meta_path = self.meta_path(id);
        if !meta_path.exists() {
            return Err(EnclaveError::NotFound { id: id.to_string() });
        }
        let meta_bytes = fs::read(&meta_path)?;
        let record: SecretRecord = serde_json::from_slice(&meta_bytes)?;
        Ok(record)
    }

    /// Find a record by name (linear scan — acceptable at secrets-manager scale).
    pub fn find_by_name(&self, name: &str) -> Result<SecretRecord, EnclaveError> {
        self.list_all()?
            .into_iter()
            .find(|r| r.name == name && r.deleted_at.is_none())
            .ok_or_else(|| EnclaveError::NotFound {
                id: name.to_string(),
            })
    }

    // ── List ──────────────────────────────────────────────────────────────────

    /// List all non-deleted secret records (metadata only).
    pub fn list(&self) -> Result<Vec<SecretRecord>, EnclaveError> {
        Ok(self
            .list_all()?
            .into_iter()
            .filter(|r| r.deleted_at.is_none())
            .collect())
    }

    fn list_all(&self) -> Result<Vec<SecretRecord>, EnclaveError> {
        let mut records = Vec::new();
        for entry in fs::read_dir(&self.base)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let bytes = fs::read(&path)?;
                let record = serde_json::from_slice::<SecretRecord>(&bytes)?;
                records.push(record);
            }
        }
        records.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(records)
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    /// Soft-delete: set `deleted_at` in metadata; blob retained for audit.
    pub fn soft_delete(&self, id: &Uuid) -> Result<(), EnclaveError> {
        let mut record = self.load_meta(id)?;
        record.deleted_at = Some(Utc::now());
        self.save_meta(&record)
    }

    /// Hard-delete: remove both metadata and blob files.
    pub fn hard_delete(&self, id: &Uuid) -> Result<(), EnclaveError> {
        let m = self.meta_path(id);
        let b = self.blob_path(id);
        if m.exists() {
            fs::remove_file(&m)?;
        }
        if b.exists() {
            fs::remove_file(&b)?;
        }
        Ok(())
    }

    // ── Existence check ───────────────────────────────────────────────────────

    pub fn exists(&self, id: &Uuid) -> bool {
        self.meta_path(id).exists()
    }

    pub fn name_exists(&self, name: &str) -> bool {
        self.find_by_name(name).is_ok()
    }

    /// NIST SP 800-88 Crypto-Shredding: overwrites file sectors with random bytes
    /// before unlinking `.blob` and `.meta.json` files.
    pub fn crypto_shred(&self, id: &Uuid) -> Result<(), EnclaveError> {
        let m = self.meta_path(id);
        let b = self.blob_path(id);

        if !m.exists() && !b.exists() {
            return Err(EnclaveError::NotFound { id: id.to_string() });
        }

        use ring::rand::{SecureRandom, SystemRandom};
        use std::fs::OpenOptions;
        use std::io::Write;

        let rng = SystemRandom::new();

        for path in [&m, &b] {
            if path.exists() {
                let metadata = fs::metadata(path)?;
                let len = metadata.len() as usize;
                let mut file = OpenOptions::new().write(true).open(path)?;
                let mut buf = vec![0u8; len];
                rng.fill(&mut buf).map_err(|_| EnclaveError::Internal)?;
                file.write_all(&buf)?;
                file.sync_all()?;
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
}
