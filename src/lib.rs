//! # renshuu-rs: A Rust Renshuu v1 API client.

mod api;

pub mod cli;
pub mod config;

use std::sync::Arc;

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};

const RENSHUU_BASE_URI: &str = "https://api.renshuu.org/v1";

/// The Renshuu API client.
#[derive(Clone)]
pub struct Renshuu {
    client: Arc<reqwest::Client>,
}

/// # Constructors
impl Renshuu {
    pub fn new(api_key: &str) -> Result<Renshuu> {
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

        Ok(Renshuu { client })
    }
}

/// # Renshuu API Methods
impl Renshuu {
    pub fn user(&self) -> api::user::UserHandler {
        api::user::UserHandler::new(self)
    }
}

/// # HTTP Methods
impl Renshuu {
    pub async fn get(&self, route: &str) -> Result<reqwest::Response> {
        let url = format!("{}{}", crate::RENSHUU_BASE_URI, route);
        Ok(self.client.get(url).send().await?)
    }
}
