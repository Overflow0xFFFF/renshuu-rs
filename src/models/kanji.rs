use serde::Deserialize;
use serde::Serialize;

use super::terms::SimpleKanji;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct KanjiSearchResponse {
    result_count: i64,
    kanjis: Vec<SimpleKanji>,
}
