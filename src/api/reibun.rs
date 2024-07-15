//! The reibun API.

use std::collections::HashMap;

use anyhow::Result;

use crate::{models::reibun::ReibunSearchResponse, Renshuu};

/// Handler for Renshuu's reibun API.
///
/// Created with [`Renshuu::reibun`].
pub struct ReibunHandler<'kao> {
    renshuu: &'kao Renshuu,
}

impl<'kao> ReibunHandler<'kao> {
    pub(crate) fn new(renshuu: &'kao Renshuu) -> Self {
        Self { renshuu }
    }

    /// Search the sentence repository.
    pub async fn search(&self, query: &str) -> Result<ReibunSearchResponse> {
        let route = "/reibun/search";
        let mut params = HashMap::new();
        params.insert("value".to_string(), query.to_string());
        self.renshuu
            .get::<ReibunSearchResponse>(route, None, Some(params))
            .await
    }

    /// Search the sentence repository by word ID.
    pub async fn search_by(&self, word_id: &str) -> Result<ReibunSearchResponse> {
        let route = format!("/reibun/search/{}", word_id);
        self.renshuu
            .get::<ReibunSearchResponse>(&route, None, None)
            .await
    }
}
