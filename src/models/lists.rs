use serde::Deserialize;
use serde::Serialize;

use super::terms::TermListContents;
use super::terms::TermTypeGroups;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct ListsResponse {
    termtype_groups: Vec<TermTypeGroups>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct ListAllByTermTypeResponse {
    contents: TermListContents,
}
