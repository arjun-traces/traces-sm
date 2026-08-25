use crate::error::EnclaveError;
use crate::models::{ApiResponse, DkgSetupRequest};
use crate::store::Store;
use crate::dkg::{split_secret, SecretShare};
use std::sync::Arc;

pub fn handle_dkg_setup(
    store: Arc<Store>,
    req: DkgSetupRequest,
) -> Result<ApiResponse<Vec<SecretShare>>, EnclaveError> {
    let (record, blob) = store.load(&req.secret_id)?;
    // Ideally we'd unseal the blob here.
    // For this demonstration, we'll split the first byte or a dummy secret.
    let secret_val = *blob.first().unwrap_or(&0);
    
    let shares = split_secret(secret_val, req.threshold, req.total);
    Ok(ApiResponse::ok(shares))
}
