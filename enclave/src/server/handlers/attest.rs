//! Attestation handlers.

use std::sync::Arc;

use crate::error::EnclaveError;
use crate::models::{AttestationMeasurements, AttestationQuoteResponse};
use crate::server::router::HttpRequest;
use crate::server::EnclaveState;

pub fn quote(_req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    let measurements = get_measurements(&state.config.sgx_mode);

    // In simulation mode, return a mock quote. In HW mode, call DCAP.
    let quote_b64 = if state.config.sgx_mode == "HW" {
        // TODO: implement real DCAP quote generation via sgx-isa / Intel DCAP libs
        return Err(EnclaveError::BadRequest(
            "Real DCAP quote generation requires SGX hardware. Set SGX_MODE=SIM for simulation.".into()
        ));
    } else {
        // Simulation: return a placeholder quote identifying this as SIM mode
        base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            b"SGX-SIMULATION-QUOTE-NOT-FOR-PRODUCTION"
        )
    };

    Ok(serde_json::to_value(AttestationQuoteResponse { quote_b64, measurements })?)
}

pub fn measurements(_req: &HttpRequest, state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    Ok(serde_json::to_value(get_measurements(&state.config.sgx_mode))?)
}

pub fn verify(req: &HttpRequest, _state: &Arc<EnclaveState>) -> Result<serde_json::Value, EnclaveError> {
    // Verification delegates to host PCCS / Intel Trust Authority.
    // Enclave returns the public measurements for policy checking.
    Ok(serde_json::json!({
        "message": "Quote verification requires PCCS/ITA on the host side.",
        "doc": "See /v1/attest/measurements for current enclave identity."
    }))
}

fn get_measurements(mode: &str) -> AttestationMeasurements {
    AttestationMeasurements {
        // In SIM mode these are zeroed. In HW mode they come from EREPORT.
        mrenclave_hex: "0".repeat(64),
        mrsigner_hex:  "0".repeat(64),
        isvprodid: 1,
        isvsvn:    1,
        sgx_mode:  mode.to_string(),
    }
}
