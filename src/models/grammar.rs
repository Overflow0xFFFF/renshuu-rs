use serde::Deserialize;
use serde::Serialize;

use super::terms::Grammar;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct GrammarSearchResponse {
    result_count: i64,
    kanjis: Vec<Grammar>,
}
