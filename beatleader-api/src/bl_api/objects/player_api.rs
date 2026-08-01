use serde::{Deserialize, Serialize};

use crate::objects::common::PlayerId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub avatar: String,
    pub rank: u64,
    pub country_rank: u64,
    pub country: String,
}
