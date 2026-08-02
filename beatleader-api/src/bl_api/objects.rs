pub mod common;
pub mod identity_api;
pub mod leaderboard_api;
pub mod player_api;
pub mod score_api;
pub mod scores_api;

pub(self) use {
    common::*,
    reqwest::Url,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    serde_repr::{Deserialize_repr, Serialize_repr},
    std::ops::{Deref, DerefMut},
};
