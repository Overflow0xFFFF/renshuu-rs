//! The grammar API.

use std::collections::HashMap;

use anyhow::Result;

use crate::models::terms::Grammar;
use crate::{models::grammar::GrammarSearchResponse, Renshuu};

/// Handler for Renshuu's grammar API.
///
/// Created with [`Renshuu::grammar`].
pub struct GrammarHandler<'kao> {
    renshuu: &'kao Renshuu,
}

impl<'kao> GrammarHandler<'kao> {
    pub(crate) fn new(renshuu: &'kao Renshuu) -> Self {
        Self { renshuu }
    }

    /// Search the grammar dictionary.
    pub async fn search(&self, query: &str) -> Result<GrammarSearchResponse> {
        let route = "/grammar/search";
        let mut params = HashMap::new();
        params.insert("value".to_string(), query.to_string());
        self.renshuu
            .get::<GrammarSearchResponse>(route, None, Some(params))
            .await
    }

    /// Look up grammar by its ID.
    pub async fn lookup(&self, id: &str) -> Result<Grammar> {
        let route = format!("/grammar/{}", id);
        self.renshuu.get::<Grammar>(&route, None, None).await
    }
}
