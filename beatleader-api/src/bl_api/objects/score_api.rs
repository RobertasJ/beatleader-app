use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    pub name: String,
    pub author: String,
    pub mapper: String,
    #[serde(alias = "coverImage")]
    pub cover: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub value: DifficultyType,
    pub stars: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: ScoreId,
    pub accuracy: f32,
    pub leaderboard: Leaderboard,
    pub pp: Pp,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub id: LeaderboardId,
    pub song: Song,
    pub difficulty: Difficulty,
}
