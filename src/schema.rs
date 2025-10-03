use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSchema {
    pub name: String,
    pub creator: String,
    pub plays: i32,
}
