use crate::error::EnclaveError;
use crate::models::{
    ApiResponse, DkgSetupRequest, FrostDkgSetupRequest, FrostCommitRequest,
    FrostSignShareRequest, FrostAggregateRequest, FrostVerifyRequest,
};
use crate::store::Store;
use crate::dkg::{split_secret_vss, SecretShare};
use crate::frost::{
    generate_dealer_keys, round1_commit, round2_sign_share, aggregate_signature,
    verify_signature, FrostKeyGenOutput, FrostRound1Output,
};
use crate::server::router::HttpRequest;
use crate::server::EnclaveState;
use std::sync::Arc;

pub fn handle_dkg_setup(
    store: Arc<Store>,
    req: DkgSetupRequest,
) -> Result<ApiResponse<Vec<SecretShare>>, EnclaveError> {
    let (_record, blob) = store.load(&req.secret_id)?;
    let secret_val = *blob.first().unwrap_or(&0);
    
    let (shares, _commitment) = split_secret_vss(secret_val, req.threshold, req.total);
    Ok(ApiResponse::ok(shares))
}

pub fn handle_frost_setup(
    req: &HttpRequest,
    _state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    let body: FrostDkgSetupRequest = serde_json::from_slice(&req.body)?;
    let output = generate_dealer_keys(body.max_signers, body.min_signers)?;
    Ok(serde_json::to_value(output)?)
}

pub fn handle_frost_commit(
    req: &HttpRequest,
    _state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    let body: FrostCommitRequest = serde_json::from_slice(&req.body)?;
    let output = round1_commit(&body.key_package_json)?;
    Ok(serde_json::to_value(output)?)
}

pub fn handle_frost_sign(
    req: &HttpRequest,
    _state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    let body: FrostSignShareRequest = serde_json::from_slice(&req.body)?;
    let msg_bytes = hex::decode(&body.message_hex)
        .map_err(|e| EnclaveError::BadRequest(format!("Invalid message hex: {}", e)))?;
    let share_json = round2_sign_share(
        &body.key_package_json,
        &body.nonces_json,
        &body.commitments_map_json,
        &msg_bytes,
    )?;
    Ok(serde_json::json!({ "signature_share_json": share_json }))
}

pub fn handle_frost_aggregate(
    req: &HttpRequest,
    _state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    let body: FrostAggregateRequest = serde_json::from_slice(&req.body)?;
    let msg_bytes = hex::decode(&body.message_hex)
        .map_err(|e| EnclaveError::BadRequest(format!("Invalid message hex: {}", e)))?;
    let sig_hex = aggregate_signature(
        &body.public_key_package_json,
        &body.commitments_map_json,
        &body.signature_shares_json,
        &msg_bytes,
    )?;
    Ok(serde_json::json!({ "signature_hex": sig_hex }))
}

pub fn handle_frost_verify(
    req: &HttpRequest,
    _state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    let body: FrostVerifyRequest = serde_json::from_slice(&req.body)?;
    let msg_bytes = hex::decode(&body.message_hex)
        .map_err(|e| EnclaveError::BadRequest(format!("Invalid message hex: {}", e)))?;
    let valid = verify_signature(&body.group_public_key_hex, &body.signature_hex, &msg_bytes)?;
    Ok(serde_json::json!({ "valid": valid }))
}
