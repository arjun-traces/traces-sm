//! WebAssembly Asynchronous HTTP API Client (`gloo-net`).
//!
//! # Responsibilities
//! Provides typed asynchronous fetch functions communicating with the host proxy
//! backend at `http://localhost:8080/v1`.

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

/// Secret record summary model for WebAssembly GUI views.
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Secret {
    /// Unique secret UUID.
    pub id: String,
    /// Human-readable secret name.
    pub name: String,
    /// Plaintext or masked secret payload representation.
    pub value: String,
}

/// Cryptographic key summary model for NIST lifecycle views.
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct KeyInfo {
    /// Unique key UUID or alias.
    pub id: String,
    /// NIST SP 800-57 lifecycle state.
    pub state: String,
    /// Key algorithm identifier (e.g. RSA-4096, ECDSA-P256, ML-KEM-768).
    pub algorithm: String,
}

/// DKG cluster peer node status model.
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct DkgNode {
    /// Node identifier string.
    pub id: String,
    /// Network address of the DKG peer.
    pub address: String,
    /// RA-TLS connection and attestation verification status.
    pub status: String,
}

/// NIST SP 800-90B DRBG continuous health telemetry model.
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct EntropyHealth {
    /// Physical or virtual entropy source identifier.
    pub source: String,
    /// Adaptive Proportion Test status (`PASSED` or `FAILED`).
    pub apt_status: String,
    /// Repetition Count Test status (`PASSED` or `FAILED`).
    pub rct_status: String,
    /// Estimated min-entropy in bits per byte.
    pub min_entropy: f32,
}

/// Base URI prefix for host API endpoints.
const API_BASE: &str = "http://localhost:8080/v1";

/// Fetches the list of sealed secrets from the host API.
pub async fn get_secrets() -> Result<Vec<Secret>, gloo_net::Error> {
    Request::get(&format!("{}/secrets", API_BASE))
        .send()
        .await?
        .json()
        .await
}

/// Fetches the list of active/inactive cryptographic keys from the host API.
pub async fn get_keys() -> Result<Vec<KeyInfo>, gloo_net::Error> {
    Request::get(&format!("{}/keys", API_BASE))
        .send()
        .await?
        .json()
        .await
}

/// Dispatches a key lifecycle transition request to the host API.
pub async fn transition_key(id: &str, new_state: &str) -> Result<(), gloo_net::Error> {
    Request::post(&format!("{}/lifecycle/transition", API_BASE))
        .json(&serde_json::json!({ "id": id, "state": new_state }))?
        .send()
        .await?;
    Ok(())
}

/// Dispatches a NIST SP 800-88 cryptographic shredding request for a key.
pub async fn shred_key(id: &str) -> Result<(), gloo_net::Error> {
    Request::post(&format!("{}/lifecycle/shred", API_BASE))
        .json(&serde_json::json!({ "id": id }))?
        .send()
        .await?;
    Ok(())
}

/// Fetches registered DKG cluster peer nodes and their RA-TLS status.
pub async fn get_dkg_nodes() -> Result<Vec<DkgNode>, gloo_net::Error> {
    Request::get(&format!("{}/dkg/nodes", API_BASE))
        .send()
        .await?
        .json()
        .await
}

/// Fetches real-time NIST SP 800-90B DRBG continuous health test results.
pub async fn get_entropy_health() -> Result<EntropyHealth, gloo_net::Error> {
    Request::get(&format!("{}/entropy/health", API_BASE))
        .send()
        .await?
        .json()
        .await
}
