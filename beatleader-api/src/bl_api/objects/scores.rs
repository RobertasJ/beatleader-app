use std::ops::{Deref, DerefMut};

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::objects::{
    LeaderboardId, ScoreId, SongId, difficulty, helpers::Pp, score::Score, song::Song,
};

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
pub struct ScoreLeaderBoard {
    pub id: LeaderboardId,
    pub song: Song,
    pub difficulty: difficulty::Difficulty,
}
