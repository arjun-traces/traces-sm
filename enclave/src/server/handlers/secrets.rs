//! Secrets CRUD handlers.

use std::sync::Arc;
use base64::Engine as _;
use chrono::Utc;
use uuid::Uuid;

use crate::crypto::{encrypt_secret, decrypt_secret};
use crate::error::EnclaveError;
use crate::models::{CreateSecretRequest, SecretMetadata, SecretResponse, UpdateSecretRequest};
use crate::server::router::HttpRequest;
use crate::server::EnclaveState;
use crate::store::SecretRecord;
use crate::zkp::schnorr::generate_commitment;

pub fn create(req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    let body: CreateSecretRequest = serde_json::from_slice(&req.body)?;

    // Check for duplicate name
    if state.store.name_exists(&body.name) {
        return Err(EnclaveError::AlreadyExists { id: body.name.clone() });
    }

    let value_bytes = base64::engine::general_purpose::STANDARD
        .decode(&body.value_b64)
        .map_err(|_| EnclaveError::BadRequest("value_b64 is not valid base64".into()))?;

    // Generate ZKP commitment for token possession proof
    let zkp_commitment = generate_commitment(&value_bytes)
        .ok()
        .map(|c| c.point_hex);

    // Encrypt the secret inside the enclave
    let blob = encrypt_secret(&value_bytes, "seal:secrets", state.provider.as_ref())?;

    let id = Uuid::new_v4();
    let now = Utc::now();
    let record = SecretRecord {
        id,
        name: body.name.clone(),
        secret_type: body.secret_type,
        version: 1,
        public_key_pem: None,
        algorithm: None,
        owner: "default".to_string(),
        tags: body.tags.unwrap_or_default(),
        created_at: now,
        updated_at: now,
        expires_at: body.expires_at,
        deleted_at: None,
        zkp_commitment,
    };

    state.store.save(&record, &blob)?;

    let meta = SecretMetadata {
        id: record.id,
        name: record.name,
        secret_type: record.secret_type,
        version: record.version,
        owner: record.owner,
        created_at: record.created_at,
        updated_at: record.updated_at,
        expires_at: record.expires_at,
        tags: record.tags,
        lifecycle_state: crate::nist::KeyLifecycleState::PreOperational,
    };

    Ok(serde_json::to_value(SecretResponse { metadata: meta, value_b64: None })?)
}

pub fn get(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| EnclaveError::BadRequest("invalid UUID".into()))?;
    let (record, blob) = state.store.load(&uuid)?;

    // Decrypt inside the enclave
    let plaintext = decrypt_secret(&blob, "seal:secrets", state.provider.as_ref())?;
    let value_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);

    let meta = SecretMetadata {
        id: record.id,
        name: record.name,
        secret_type: record.secret_type,
        version: record.version,
        owner: record.owner,
        created_at: record.created_at,
        updated_at: record.updated_at,
        expires_at: record.expires_at,
        tags: record.tags,
        lifecycle_state: crate::nist::KeyLifecycleState::Operational,
    };

    Ok(serde_json::to_value(SecretResponse { metadata: meta, value_b64: Some(value_b64) })?)
}

pub fn list(_req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    let records = state.store.list()?;
    let metas: Vec<SecretMetadata> = records.into_iter().map(|r| SecretMetadata {
        id: r.id,
        name: r.name,
        secret_type: r.secret_type,
        version: r.version,
        owner: r.owner,
        created_at: r.created_at,
        updated_at: r.updated_at,
        expires_at: r.expires_at,
        tags: r.tags,
        lifecycle_state: crate::nist::KeyLifecycleState::Operational,
    }).collect();
    Ok(serde_json::to_value(metas)?)
}

pub fn update(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| EnclaveError::BadRequest("invalid UUID".into()))?;
    let body: UpdateSecretRequest = serde_json::from_slice(&req.body)?;

    let (mut record, _old_blob) = state.store.load(&uuid)?;

    let value_bytes = base64::engine::general_purpose::STANDARD
        .decode(&body.value_b64)
        .map_err(|_| EnclaveError::BadRequest("value_b64 is not valid base64".into()))?;

    let new_blob = encrypt_secret(&value_bytes, "seal:secrets", state.provider.as_ref())?;
    record.version += 1;
    record.updated_at = Utc::now();
    record.zkp_commitment = generate_commitment(&value_bytes).ok().map(|c| c.point_hex);

    state.store.save(&record, &new_blob)?;
    Ok(serde_json::json!({ "id": uuid, "version": record.version, "updated_at": record.updated_at }))
}

pub fn delete(_req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| EnclaveError::BadRequest("invalid UUID".into()))?;
    state.store.soft_delete(&uuid)?;
    Ok(serde_json::json!({ "id": uuid, "deleted": true }))
}
