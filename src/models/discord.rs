use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Discord {
    pub discord_id: Option<i64>,
    pub discord_name: Option<String>,
    pub joined_at: Option<String>,
    pub balance: Option<i64>,
}