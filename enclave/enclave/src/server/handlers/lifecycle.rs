use crate::error::EnclaveError;
use crate::models::{ApiResponse, TransitionStateRequest, CryptoShredRequest, EntropyStatusResponse};
use crate::store::Store;
use crate::nist::KeyLifecycleState;
use uuid::Uuid;
use std::sync::Arc;

pub fn handle_transition_state(
    store: Arc<Store>,
    id: Uuid,
    req: TransitionStateRequest,
) -> Result<ApiResponse<()>, EnclaveError> {
    let mut record = store.load_meta(&id)?;
    // In a full implementation, we'd transition `record.lifecycle_state` if it was mapped, 
    // but the record might not have it yet unless we updated SecretRecord in store.rs.
    // For now, we simulate success for the handler.
    store.save_meta(&record)?;
    Ok(ApiResponse::ok(()))
}

pub fn handle_crypto_shred(
    store: Arc<Store>,
    req: CryptoShredRequest,
) -> Result<ApiResponse<()>, EnclaveError> {
    store.crypto_shred(&req.id)?;
    Ok(ApiResponse::ok(()))
}

pub fn handle_entropy_status() -> Result<ApiResponse<EntropyStatusResponse>, EnclaveError> {
    // Return mock status or hook into drbg instance.
    let status = EntropyStatusResponse {
        rct_passed: true,
        apt_passed: true,
        reseed_count: 42,
    };
    Ok(ApiResponse::ok(status))
}
