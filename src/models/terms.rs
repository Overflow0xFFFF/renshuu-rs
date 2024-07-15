use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::privacy::PrivacySetting;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Grammar {
    id: String,
    title_english: String,
    title_japanese: String,
    url: String,
    meaning: HashMap<String, String>,
    meaning_long: HashMap<String, String>,
    presence: Vec<Presences>,
    user_data: TermUserData,
    models: Vec<GrammarModel>,
    construct: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct GrammarModel {
    japanese: String,
    hiragana: String,
    meanings: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Kanji {
    id: String,
    kanji: String,
    scount: String,
    definition: String,
    radical: String,
    radical_name: String,
    onyomi: String,
    kunyomi: String,
    // Undocumented!
    onyokunyomimi: String,
    kanken: String,
    jlpt: String,
    rwords: Option<Vec<KanjiRelatedWords>>,
    parts: Option<Vec<KanjiParts>>,
    presence: Option<Vec<Presences>>,
    user_data: Option<TermUserData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct KanjiParts {
    piece: String,
    definition: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct KanjiRelatedWords {
    reading: String,
    words: Vec<SimpleWord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct ListPresence {
    list_id: i64,
    name: String,
    set_name: String,
    #[serde(rename = "hasWord")]
    has_word: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Presences {
    scheds: Vec<SchedulePresence>,
    lists: Vec<ListPresence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct SchedulePresence {
    sched_id: i64,
    name: String,
    #[serde(rename = "hasWord")]
    has_word: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct SimpleSentence {
    id: i64,
    japanese: String,
    hiragana: String,
    meaning: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct SimpleWord {
    term: String,
    def: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct StudyVector {
    name: String,
    correct_count: i64,
    missed_count: i64,
    mastery_perc: i64,
    last_quizzed: String,
    next_quiz: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Term {
    Grammar(Grammar),
    Kanji(Kanji),
    Sentence(SimpleSentence),
    Word(Word),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct TermList {
    list_id: String,
    title: String,
    description: String,
    termtype: TermType,
    num_terms: String,
    privacy: PrivacySetting,
    contents: Option<TermListContents>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct TermListContents {
    result_count: i64,
    total_pg: i64,
    per_pg: i64,
    pg: i64,
    group: Option<String>,
    terms: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TermType {
    Grammar,
    Kanji,
    #[serde(rename = "sent")]
    Sentences,
    Vocab,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct TermTypeGroup {
    list_count: i64,
    group_title: String,
    lists: Vec<TermList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct TermTypeGroups {
    termtype: TermType,
    list_count: i64,
    groups: Vec<TermTypeGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct TermUserData {
    correct_count: i64,
    missed_count: i64,
    mastery_avg_perc: String,
    study_vectors: HashMap<String, StudyVector>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Word {
    is_common: bool,
    kanji_full: String,
    hiragana_full: String,
    id: String,
    reibuns: String,
    aforms: Vec<String>,
    edict_ent: String,
    pic: Vec<String>,
    config: Vec<String>,
    markers: Vec<String>,
    pitch: Vec<String>,
    notes: Vec<String>,
    typeofspeech: String,
    presence: Vec<Presences>,
    user_data: TermUserData,
    def: Vec<String>,
}
