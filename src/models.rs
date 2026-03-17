use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateMessage {
    pub content: String,
}

#[derive(Serialize, FromRow)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}