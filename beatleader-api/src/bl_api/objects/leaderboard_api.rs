use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub id: LeaderboardId,
    pub song: Song,
    pub difficulty: LeaderboardDifficulty,
    pub scores: Vec<Score>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: ScoreId,
    pub accuracy: f32,
    pub pp: Pp,
    pub rank: u32,
    pub player: Player,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    pub name: String,
    pub author: String,
    pub mapper: String,
    #[serde(alias = "coverImage")]
    pub cover: Url,
    pub difficulties: Vec<SongDifficulty>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongDifficulty {
    pub mode: i32,
    pub value: DifficultyType,
    pub stars: Option<f32>,
    pub difficulty_name: String,
    pub song_id: SongId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardDifficulty {
    pub value: DifficultyType,
    pub difficulty_name: String,
    pub stars: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub country: String,
    pub avatar: Url,
}
