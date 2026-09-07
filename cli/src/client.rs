use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Client {
    base_url: String,
    http: ReqwestClient,
}

pub type ApiClient = Client;

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: ReqwestClient::new(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.http.get(&url).send().await?.json::<T>().await?;
        Ok(res)
    }

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
