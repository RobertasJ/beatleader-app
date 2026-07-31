use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::objects::{LeaderboardId, difficulty::Difficulty, score::PartialScore, song::Song};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub id: LeaderboardId,
    pub song: Song,
    pub difficulty: Difficulty,
    pub scores: Vec<PartialScore>,
}
