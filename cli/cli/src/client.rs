use reqwest::Client as ReqwestClient;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Client {
    base_url: String,
    http: ReqwestClient,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            http: ReqwestClient::new(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.http.get(&url).send().await?.json::<T>().await?;
        Ok(res)
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.http.post(&url).json(body).send().await?.json::<T>().await?;
        Ok(res)
    }
}
