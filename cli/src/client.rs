//! Asynchronous HTTP/REST API Client for Host and Enclave Interactions.
//!
//! # Architecture & Operator Model
//! Provides a typed async wrapper around `reqwest::Client` for interacting with the
//! `traces-sm-host` API server.
//!
//! # Responsibilities
//! - **Connection Reuse**: Maintains internal HTTP connection pooling via [`reqwest::Client`].
//! - **Serialization & Deserialization**: Handles generic JSON payload transmission and response decoding.
//! - **Error Propagation**: Converts network failures, serialization mismatches, and HTTP error
//!   statuses into [`anyhow::Result`].

use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Asynchronous API client for communicating with the `traces-sm` host gateway.
pub struct Client {
    /// Base URL of the target host API service (e.g., `http://localhost:8080`).
    base_url: String,
    /// Internal connection-pooled HTTP client instance.
    http: ReqwestClient,
}

/// Type alias for [`Client`].
pub type ApiClient = Client;

impl Client {
    /// Constructs a new [`Client`] instance targeting the specified base URL.
    ///
    /// # Arguments
    /// * `base_url` - Root URI prefix for all outgoing API requests (e.g. `http://127.0.0.1:8080`).
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: ReqwestClient::new(),
        }
    }

    /// Executes an asynchronous HTTP `GET` request against the target path.
    ///
    /// # Type Parameters
    /// * `T` - Deserializable response type implementing [`serde::de::DeserializeOwned`].
    ///
    /// # Arguments
    /// * `path` - Relative endpoint path, including query string (e.g., `/v1/secrets?name=db_key`).
    ///
    /// # Errors
    /// Returns an error if network transmission fails or if the response JSON cannot be decoded into `T`.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.http.get(&url).send().await?.json::<T>().await?;
        Ok(res)
    }

    /// Executes an asynchronous HTTP `POST` request with a JSON body against the target path.
    ///
    /// # Type Parameters
    /// * `T` - Deserializable response type implementing [`serde::de::DeserializeOwned`].
    /// * `B` - Serializable request payload type implementing [`serde::Serialize`].
    ///
    /// # Arguments
    /// * `path` - Relative endpoint path (e.g., `/v1/secrets`).
    /// * `body` - Request payload reference to be serialized to JSON.
    ///
    /// # Errors
    /// Returns an error if JSON serialization fails, network request fails, or response parsing fails.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self
            .http
            .post(&url)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(res)
    }
}
