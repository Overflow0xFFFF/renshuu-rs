//! The word API.

use std::collections::HashMap;

use anyhow::Result;

use crate::{models::word::WordSearchResponse, Renshuu};

/// Handler for Renshuu's word API.
///
/// Created with [`Renshuu::word`].
pub struct WordHandler<'kao> {
    renshuu: &'kao Renshuu,
}

impl<'kao> WordHandler<'kao> {
    pub(crate) fn new(renshuu: &'kao Renshuu) -> Self {
        Self { renshuu }
    }

    /// Search the vocab dictionary.
    pub async fn search(&self, query: &str) -> Result<WordSearchResponse> {
        let route = "/word/search";
        let mut params = HashMap::new();
        params.insert("value".to_string(), query.to_string());
        self.renshuu
            .get::<WordSearchResponse>(route, None, Some(params))
            .await
    }

    /// Lookup the word in the dictionary by its ID.
    pub async fn lookup(&self, id: &str) -> Result<WordSearchResponse> {
        let route = format!("/word/{}", id);
        self.renshuu
            .get::<WordSearchResponse>(&route, None, None)
            .await
    }
}
