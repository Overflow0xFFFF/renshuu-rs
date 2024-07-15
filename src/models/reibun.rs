use serde::Deserialize;
use serde::Serialize;

use super::terms::SimpleSentence;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct ReibunSearchResponse {
    result_count: i64,
    per_page: i64,
    pg: i64,
    reibuns: Vec<SimpleSentence>,
}
