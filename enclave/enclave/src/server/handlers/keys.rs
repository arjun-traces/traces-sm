//! Key pair operation handlers.

use std::sync::Arc;
use base64::Engine as _;
use chrono::Utc;
use uuid::Uuid;

use crate::error::EnclaveError;
use crate::keygen;
use crate::models::{
    DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse,
    GenerateKeyRequest, KeyAlgorithm, KeyResponse, SignRequest, SignResponse,
    VerifyRequest, VerifyResponse,
};
use crate::server::router::HttpRequest;
use crate::server::EnclaveState;
use crate::store::SecretRecord;
use crate::models::SecretType;

pub fn generate(req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    let body: GenerateKeyRequest = serde_json::from_slice(&req.body)?;

    if state.store.name_exists(&body.name) {
        return Err(EnclaveError::AlreadyExists { id: body.name.clone() });
    }

    // Generate inside the enclave — private key sealed immediately
    let (pub_pem, sealed_priv) = keygen::generate_keypair(body.algorithm, state.provider.as_ref())?;

    let id = Uuid::new_v4();
    let now = Utc::now();
    let record = SecretRecord {
        id,
        name: body.name.clone(),
        secret_type: SecretType::AsymmetricKey,
        version: 1,
        public_key_pem: Some(pub_pem.clone()),
        algorithm: Some(body.algorithm.to_string()),
        owner: "default".to_string(),
        tags: body.tags.unwrap_or_default(),
        created_at: now,
        updated_at: now,
        expires_at: None,
        deleted_at: None,
        zkp_commitment: None,
    };

    state.store.save(&record, &sealed_priv)?;

    let metadata = crate::models::KeyMetadata {
        id,
        name: body.name,
        algorithm: body.algorithm,
        created_at: now,
        lifecycle_state: crate::nist::KeyLifecycleState::PreOperational,
    };
    Ok(serde_json::to_value(KeyResponse {
        metadata,
        public_key_pem: pub_pem,
    })?)
}

pub fn list(_req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    let records = state.store.list()?.into_iter()
        .filter(|r| r.secret_type == SecretType::AsymmetricKey)
        .map(|r| serde_json::json!({
            "id": r.id,
            "name": r.name,
            "algorithm": r.algorithm,
            "public_key_pem": r.public_key_pem,
            "created_at": r.created_at,
        }))
        .collect::<Vec<_>>();
    Ok(serde_json::Value::Array(records))
}

pub fn public_key(_req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    let record = state.store.load_meta(&uuid)?;
    Ok(serde_json::json!({ "id": uuid, "public_key_pem": record.public_key_pem }))
}

pub fn sign(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    let body: SignRequest = serde_json::from_slice(&req.body)?;
    let (record, sealed_priv) = state.store.load(&uuid)?;

    let message = base64::engine::general_purpose::STANDARD
        .decode(&body.message_b64)
        .map_err(|_| EnclaveError::BadRequest("bad message_b64".into()))?;

    let algorithm: KeyAlgorithm = serde_json::from_value(
        serde_json::to_value(record.algorithm.as_deref().unwrap_or("")).unwrap_or_default()
    ).map_err(|_| EnclaveError::UnsupportedAlgorithm(record.algorithm.unwrap_or_default()))?;

    let sig = keygen::sign(algorithm, &sealed_priv, &message, state.provider.as_ref())?;
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(&sig);

    Ok(serde_json::to_value(SignResponse { signature_b64: sig_b64, algorithm })?)
}

pub fn verify(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    let body: VerifyRequest = serde_json::from_slice(&req.body)?;
    let record = state.store.load_meta(&uuid)?;

    let message = base64::engine::general_purpose::STANDARD
        .decode(&body.message_b64)
        .map_err(|_| EnclaveError::BadRequest("bad message_b64".into()))?;
    let signature = base64::engine::general_purpose::STANDARD
        .decode(&body.signature_b64)
        .map_err(|_| EnclaveError::BadRequest("bad signature_b64".into()))?;

    let algorithm: KeyAlgorithm = serde_json::from_value(
        serde_json::to_value(record.algorithm.as_deref().unwrap_or("")).unwrap_or_default()
    ).map_err(|_| EnclaveError::UnsupportedAlgorithm(record.algorithm.unwrap_or_default()))?;

    let pub_pem = record.public_key_pem.ok_or(EnclaveError::Internal)?;
    let valid = keygen::verify_signature(algorithm, &pub_pem, &message, &signature)?;

    Ok(serde_json::to_value(VerifyResponse { valid })?)
}

pub fn encrypt(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    let body: EncryptRequest = serde_json::from_slice(&req.body)?;
    let record = state.store.load_meta(&uuid)?;

    let plaintext = base64::engine::general_purpose::STANDARD
        .decode(&body.plaintext_b64)
        .map_err(|_| EnclaveError::BadRequest("bad plaintext_b64".into()))?;

    let pub_pem = record.public_key_pem.ok_or(EnclaveError::Internal)?;
    let ct = keygen::rsa_encrypt(&pub_pem, &plaintext)?;
    let ct_b64 = base64::engine::general_purpose::STANDARD.encode(&ct);

    Ok(serde_json::to_value(EncryptResponse { ciphertext_b64: ct_b64 })?)
}

pub fn decrypt(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    let body: DecryptRequest = serde_json::from_slice(&req.body)?;
    let (_, sealed_priv) = state.store.load(&uuid)?;

    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(&body.ciphertext_b64)
        .map_err(|_| EnclaveError::BadRequest("bad ciphertext_b64".into()))?;

    let pt = keygen::rsa_decrypt(&sealed_priv, &ciphertext, state.provider.as_ref())?;
    let pt_b64 = base64::engine::general_purpose::STANDARD.encode(&pt);

    Ok(serde_json::to_value(crate::models::DecryptResponse { plaintext_b64: pt_b64 })?)
}

pub fn rotate(req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    let (mut record, _old_priv) = state.store.load(&uuid)?;

    let algorithm: KeyAlgorithm = serde_json::from_value(
        serde_json::to_value(record.algorithm.as_deref().unwrap_or("")).unwrap_or_default()
    ).map_err(|_| EnclaveError::UnsupportedAlgorithm(record.algorithm.clone().unwrap_or_default()))?;

    let (new_pub_pem, new_sealed_priv) = keygen::generate_keypair(algorithm, state.provider.as_ref())?;

    record.public_key_pem = Some(new_pub_pem.clone());
    record.version += 1;
    record.updated_at = Utc::now();

    state.store.save(&record, &new_sealed_priv)?;

    Ok(serde_json::json!({
        "id": uuid,
        "version": record.version,
        "public_key_pem": new_pub_pem,
        "rotated_at": record.updated_at,
    }))
}

pub fn delete(_req: &HttpRequest, state: &Arc<EnclaveState>, id: &str) -> Result<serde_json::Value, EnclaveError> {
    let uuid = Uuid::parse_str(id).map_err(|_| EnclaveError::BadRequest("bad UUID".into()))?;
    state.store.hard_delete(&uuid)?;
    Ok(serde_json::json!({ "id": uuid, "deleted": true }))
}
