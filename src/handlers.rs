use axum::{Json};
use chrono::Utc;
use crate::models::Message;


pub async fn get_messages() -> Json<Vec<Message>> {
    Json(vec![
        Message { id: 0, content: "Hello bruv!".into(), created_at: Utc::now() },
    ])
}
