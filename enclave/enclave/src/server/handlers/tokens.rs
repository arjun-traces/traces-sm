//! Token handlers.

use std::sync::Arc;
use chrono::{Duration, Utc};

use crate::error::EnclaveError;
use crate::models::{CreateTokenRequest, TokenResponse};
use crate::server::router::HttpRequest;
use crate::server::EnclaveState;

pub fn create(req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    let body: CreateTokenRequest = serde_json::from_slice(&req.body)?;
    let ttl = body.ttl_secs.unwrap_or(3600);

    let (jti, jwt) = state.token_service.issue_token(
        &body.subject,
        body.scopes,
        ttl,
        state.provider.as_ref(),
    )?;

    let expires_at = Utc::now() + Duration::seconds(ttl as i64);
    Ok(serde_json::to_value(TokenResponse { token_id: jti, jwt, expires_at })?)
}

pub fn list(_req: &HttpRequest, _state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    // Token listing is maintained by the host (Python) which tracks issued tokens.
    // The enclave only issues and revokes.
    Ok(serde_json::json!({ "message": "Token listing is managed by the host API at /v1/tokens" }))
}

pub fn revoke(req: &HttpRequest, state: &Arc<EnclaveState>, jti: &str) -> Result<serde_json::Value, EnclaveError> {
    state.token_service.revoke_token(jti);
    Ok(serde_json::json!({ "token_id": jti, "revoked": true }))
}
