use crate::objects::{
    Leaderboard, PartialLeaderboard, ScoreId, difficulty::Difficulty, helpers::Pp, song::Song,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialScore {
    pub id: ScoreId,
    pub accuracy: f32,
    pub pp: Pp,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: ScoreId,
    pub accuracy: f32,
    pub leaderboard: PartialLeaderboard,
    pub pp: Pp,
    pub rank: u32,
}
