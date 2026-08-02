use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pp(Option<f32>);

impl Pp {
    pub fn new(pp: Option<f32>) -> Self {
        Self(pp)
    }

    pub fn pp(&self) -> Option<f32> {
        self.0
    }
}

impl Serialize for Pp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.0.unwrap_or(0.0))
    }
}

impl<'de> Deserialize<'de> for Pp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;

        Ok(Self(match value {
            0.0 => None,
            x => Some(x),
        }))
    }
}
