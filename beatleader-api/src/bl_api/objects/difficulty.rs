use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub value: DifficultyType,
    pub stars: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
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
