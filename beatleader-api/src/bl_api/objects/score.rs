use crate::objects::{Leaderboard, ScoreId, difficulty::Difficulty, helpers::Pp, song::Song};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialScore {
    pub id: ScoreId,
    pub accuracy: f32,
    pub pp: Pp,
    pub rank: u32,
    pub song: Song,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: ScoreId,
    pub accuracy: f32,
    pub leaderboard: Leaderboard,
    pub pp: Pp,
    pub rank: u32,
    pub song: Song,
    pub difficulty: Difficulty,
}
