use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSchema {
    pub name: String,
    pub creator: String,
    pub plays: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGameSchema {
    pub name: Option<String>,
    pub creator: Option<String>,
    pub plays: Option<i32>,
}
