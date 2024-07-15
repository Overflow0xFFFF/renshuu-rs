//! # renshuu-rs: A Rust Renshuu v1 API client.

mod api;

pub mod cli;
pub mod config;
pub mod models;

use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;

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
    pub fn grammar(&self) -> api::grammar::GrammarHandler {
        api::grammar::GrammarHandler::new(self)
    }

    pub fn kanji(&self) -> api::kanji::KanjiHandler {
        api::kanji::KanjiHandler::new(self)
    }

    pub fn reibun(&self) -> api::reibun::ReibunHandler {
        api::reibun::ReibunHandler::new(self)
    }

    pub fn user(&self) -> api::user::UserHandler {
        api::user::UserHandler::new(self)
    }
}

/// # HTTP Methods
impl Renshuu {
    pub async fn delete<U: DeserializeOwned>(
        &self, route: &str, headers: Option<HeaderMap>, params: Option<HashMap<String, String>>,
    ) -> Result<U> {
        self.request::<(), U>(reqwest::Method::DELETE, route, headers, params, None)
            .await
    }

    pub async fn get<U: DeserializeOwned>(
        &self, route: &str, headers: Option<HeaderMap>, params: Option<HashMap<String, String>>,
    ) -> Result<U> {
        self.request::<(), U>(reqwest::Method::GET, route, headers, params, None)
            .await
    }

    pub async fn post<T: Serialize, U: DeserializeOwned>(
        &self, route: &str, headers: Option<HeaderMap>, params: Option<HashMap<String, String>>,
        body: Option<T>,
    ) -> Result<U> {
        self.request::<T, U>(reqwest::Method::POST, route, headers, params, body)
            .await
    }

    pub async fn put<T: Serialize, U: DeserializeOwned>(
        &self, route: &str, headers: Option<HeaderMap>, params: Option<HashMap<String, String>>,
        body: Option<T>,
    ) -> Result<U> {
        self.request::<T, U>(reqwest::Method::PUT, route, headers, params, body)
            .await
    }

    async fn request<T: Serialize, U: DeserializeOwned>(
        &self, method: reqwest::Method, route: &str, headers: Option<HeaderMap>,
        params: Option<HashMap<String, String>>, body: Option<T>,
    ) -> Result<U> {
        let url = format!("{}{}", crate::RENSHUU_BASE_URI, route);
        let mut request = self.client.request(method, &url);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        if let Some(params) = params {
            request = request.query(&params);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send().await?.json::<U>().await?;
        Ok(response)
    }
}
