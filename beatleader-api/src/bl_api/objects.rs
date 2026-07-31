use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub use difficulty::*;
pub use leaderboard::*;
pub use player::*;
pub use score::*;
pub use scores::*;
pub use song::*;

mod difficulty;
mod helpers;
mod leaderboard;
mod player;
mod score;
mod scores;
mod song;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub id: PlayerId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ScoreId(pub(crate) i32);

impl Deref for ScoreId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct LeaderboardId(pub(crate) String);

impl Deref for LeaderboardId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct PlayerId(pub(crate) String);

impl Deref for PlayerId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct SongId(pub(crate) String);

impl Deref for SongId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
