use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Fetcher {
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MessageCreationData {
    pub chat_id: i32,
    pub content: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub author_id: i32,
    pub author: String,
    pub content: String,
    pub is_edited: bool,
    pub time_created: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ChatCreationData {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub participants: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ChatEditData {
    pub id: i32,
    pub title: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ChatWithLastMessage {
    pub chat_id: i32,
    pub title: Option<String>,
    pub last_message_author: Option<String>,
    pub chat_created_at: chrono::NaiveDateTime,
    pub last_message: Option<String>,
    pub message_created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MsgPaginatorQuery {
    pub chat_id: i32,

    pub page_number: i32,
    pub limit: i64,
    pub offset: i64,
    pub token: String,

    // TODO: <future> Добавить систему пейджинации по id последнего сообщения
    pub before: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Paginator {
    pub page: i32,
    pub limit: i64,
    pub offset: i64,
    
    // TODO: <future> Добавить систему пейджинации по id последнего сообщения
    pub before: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct RegisterData{
    pub username: String,
    pub email: String,
    pub tag: Option<String>, // Опционален, но будет генерироваться уникальный тег при пустом поле
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct LoginData{
    pub username: String, // может быть email или tag
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user_id
    pub exp: usize, // срок действия
}
