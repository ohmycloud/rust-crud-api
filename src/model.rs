use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// Debug: Allows us to print the struct for debugging purposes.
// So we can use println!("{:?}", game) to see the struct’s contents.
// Deserialize: Allows converting JSON (or other formats) into our struct.
// This is needed when receiving data from HTTP requests to convert that into our struct.
// Serialize: Allows converting our struct into JSON (or other formats).
// Used to send data in HTTP responses.
// sqlx::FromRow: Allows converting database rows into our struct.
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub plays: i32,
    pub created_at: DateTime<Utc>,
}
