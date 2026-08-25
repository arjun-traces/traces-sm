use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Secret {
    pub id: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct KeyInfo {
    pub id: String,
    pub state: String,
    pub algorithm: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct DkgNode {
    pub id: String,
    pub address: String,
    pub status: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct EntropyHealth {
    pub source: String,
    pub apt_status: String,
    pub rct_status: String,
    pub min_entropy: f32,
}

const API_BASE: &str = "http://localhost:8080/v1";

pub async fn get_secrets() -> Result<Vec<Secret>, gloo_net::Error> {
    Request::get(&format!("{}/secrets", API_BASE))
        .send()
        .await?
        .json()
        .await
}

pub async fn get_keys() -> Result<Vec<KeyInfo>, gloo_net::Error> {
    Request::get(&format!("{}/keys", API_BASE))
        .send()
        .await?
        .json()
        .await
}

pub async fn transition_key(id: &str, new_state: &str) -> Result<(), gloo_net::Error> {
    Request::post(&format!("{}/lifecycle/transition", API_BASE))
        .json(&serde_json::json!({ "id": id, "state": new_state }))?
        .send()
        .await?;
    Ok(())
}

pub async fn shred_key(id: &str) -> Result<(), gloo_net::Error> {
    Request::post(&format!("{}/lifecycle/shred", API_BASE))
        .json(&serde_json::json!({ "id": id }))?
        .send()
        .await?;
    Ok(())
}

pub async fn get_dkg_nodes() -> Result<Vec<DkgNode>, gloo_net::Error> {
    Request::get(&format!("{}/dkg/nodes", API_BASE))
        .send()
        .await?
        .json()
        .await
}

pub async fn get_entropy_health() -> Result<EntropyHealth, gloo_net::Error> {
    Request::get(&format!("{}/entropy/health", API_BASE))
        .send()
        .await?
        .json()
        .await
}
