//! The kanji API.

use std::collections::HashMap;

use anyhow::Result;

use crate::models::terms::Kanji;
use crate::{models::kanji::KanjiSearchResponse, Renshuu};

/// Handler for Renshuu's kanji API.
///
/// Created with [`Renshuu::kanji`].
pub struct KanjiHandler<'kao> {
    renshuu: &'kao Renshuu,
}

impl<'kao> KanjiHandler<'kao> {
    pub(crate) fn new(renshuu: &'kao Renshuu) -> Self {
        Self { renshuu }
    }

    /// Search the kanji dictionary.
    pub async fn search(&self, query: &str) -> Result<KanjiSearchResponse> {
        let route = "/kanji/search";
        let mut params = HashMap::new();
        params.insert("value".to_string(), query.to_string());
        self.renshuu
            .get::<KanjiSearchResponse>(route, None, Some(params))
            .await
    }

    /// Look up a kanji character in the dictionary.
    pub async fn lookup(&self, kanji: &str) -> Result<Kanji> {
        let route = format!("/kanji/{}", kanji);
        self.renshuu.get::<Kanji>(&route, None, None).await
    }
}
