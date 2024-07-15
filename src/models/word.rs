use serde::Deserialize;
use serde::Serialize;

use super::terms::Word;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct WordSearchResponse {
    result_count: i64,
    total_page: i64,
    per_page: i64,
    pg: i64,
    query: String,
    words: Vec<Word>,
}
