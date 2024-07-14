use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Profile {
    id: String,
    real_name: String,
    adventure_level: String,
    user_length: String,
    kao: String,
    studied: StudiedStats,
    level_progress_percs: LevelProgressPercentages,
    streaks: StudyStreaks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct StudiedStats {
    today_all: i64,
    today_vocab: i64,
    today_grammar: i64,
    today_kanji: i64,
    today_sent: i64,
    today_conj: i64,
    today_aconj: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct LevelProgressPercentages {
    vocab: LevelProgressVocab,
    kanji: LevelProgressJLPT,
    grammar: LevelProgressJLPT,
    sent: LevelProgressJLPT,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct LevelProgressVocab {
    n1: i64,
    n2: i64,
    n3: i64,
    n4: i64,
    n5: i64,
    n6: Option<i64>,
    kana: Option<i64>,
    kata: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct LevelProgressJLPT {
    n1: i64,
    n2: i64,
    n3: i64,
    n4: i64,
    n5: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct StudyStreaks {
    vocab: StudyStreak,
    kanji: StudyStreak,
    grammar: StudyStreak,
    sent: StudyStreak,
    conj: StudyStreak,
    aconj: StudyStreak,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct StudyStreak {
    correct_in_a_row: String,
    correct_in_a_row_alltime: String,
    days_studied_in_a_row: String,
    days_studied_in_a_row_alltime: String,
}
