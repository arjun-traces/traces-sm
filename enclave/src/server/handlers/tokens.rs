//! Token handlers.

use chrono::{Duration, Utc};
use std::sync::Arc;

use crate::error::EnclaveError;
use crate::models::{CreateTokenRequest, TokenResponse};
use crate::server::router::HttpRequest;
use crate::server::EnclaveState;

pub fn create(
    req: &HttpRequest,
    state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    let body: CreateTokenRequest = serde_json::from_slice(&req.body)?;
    let ttl = body.ttl_seconds.unwrap_or(3600);

    let (jti, jwt) = state.token_service.issue_token(
        &body.subject,
        body.scopes,
        ttl,
        state.provider.as_ref(),
    )?;

    let expires_at = Utc::now() + Duration::seconds(ttl as i64);
    Ok(serde_json::to_value(TokenResponse {
        token: jwt.clone(),
        token_id: Some(jti),
        jwt: Some(jwt),
        expires_at,
    })?)
}

pub fn list(
    _req: &HttpRequest,
    _state: &Arc<EnclaveState>,
) -> Result<serde_json::Value, EnclaveError> {
    // Token listing is maintained by the host which tracks issued tokens.
    // The enclave only issues and revokes.
    Ok(serde_json::json!({ "message": "Token listing is managed by the host API at /v1/tokens" }))
}

pub fn revoke(
    _req: &HttpRequest,
    state: &Arc<EnclaveState>,
    jti: &str,
) -> Result<serde_json::Value, EnclaveError> {
    state.token_service.revoke_token(jti);
    Ok(serde_json::json!({ "token_id": jti, "revoked": true }))
}
