use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DifficultyType {
    Easy = 1,
    Normal = 3,
    Hard = 5,
    Expert = 7,
    ExpertPlus = 9,
}

impl DifficultyType {
    pub fn color_hex(&self) -> &'static str {
        match self {
            DifficultyType::Easy => "#3CB371",
            DifficultyType::Normal => "#59b0ff",
            DifficultyType::Hard => "#ff6347",
            DifficultyType::Expert => "#bf2a42",
            DifficultyType::ExpertPlus => "#8f48db",
        }
    }
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

impl LeaderboardId {
    pub fn from_song_id_difficulty_value_and_mode(
        song_id: &SongId,
        difficulty_value: DifficultyType,
        mode: i32,
    ) -> Self {
        Self(format!(
            "{}{}{mode}",
            song_id.deref(),
            difficulty_value as u8
        ))
    }
}

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
