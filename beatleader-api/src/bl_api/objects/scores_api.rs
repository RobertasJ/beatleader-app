use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    #[serde(rename = "data")]
    scores: Vec<Score>,
}

impl Deref for Scores {
    type Target = Vec<Score>;

    fn deref(&self) -> &Self::Target {
        &self.scores
    }
}

impl DerefMut for Scores {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scores
    }
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
