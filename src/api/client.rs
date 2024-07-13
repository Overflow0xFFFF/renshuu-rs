use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

pub struct Client {
    url: String,
    client: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(url: &str, api_key: &str) -> Result<Client> {
        let url = url.to_string();
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {api_key}").as_str())?,
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str("application/json")?,
        );
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_str("application/json")?,
        );

        // set default client operation
        let client = Arc::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .user_agent("Rust")
                .build()?,
        );

        Ok(Client { url, client })
    }

    pub async fn user_profile(&self) -> Result<String> {
        let url = format!("{}/profile", self.url);
        Ok(self.client.get(url).send().await?.text().await?)
    }
}
