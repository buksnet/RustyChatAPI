use crate::AppState;
use crate::error::AppError;
use crate::models::{ChatCreationData, ChatWithLastMessage, Fetcher, LoginData, Message, MessageCreationData, MsgPaginatorQuery, RegisterData};
use crate::repositories::auth::{get_new_token, register_new_user};
use crate::repositories::chat::{
    create_new_chat, create_new_message, get_chats_with_last_message, get_messages_for_user,
};
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use serde_json::{Value, json};

pub async fn get_chats(
    State(state): State<AppState>,
    query: Query<Fetcher>,
) -> Result<Json<Vec<ChatWithLastMessage>>, AppError> {
    let user_id = query.id;
    let chats = get_chats_with_last_message(&state.db, user_id).await?;
    Ok(Json(chats))
}

pub async fn new_chat(
    State(state): State<AppState>,
    Json(chat_data): Json<ChatCreationData>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    assert!(chat_data.participants.len() >= 1);
    let db_response = create_new_chat(&state.db, chat_data).await?;
    Ok(db_response)
}

pub async fn new_message(
    State(state): State<AppState>,
    Json(message_data): Json<MessageCreationData>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let msg_id = create_new_message(&state.db, message_data).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"status": "success", "new_message_id": msg_id})),
    ))
}

pub async fn fetch_messages(
    State(state): State<AppState>,
    Query(query): Query<MsgPaginatorQuery>,
) -> Result<(StatusCode, Json<Vec<Message>>), AppError> {
    Ok((
        StatusCode::OK,
        Json(get_messages_for_user(&state.db, query).await?),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(message_data): Json<LoginData>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    get_new_token(&state.db, message_data).await
}

pub async fn register(
    State(state): State<AppState>,
    Json(message_data): Json<RegisterData>,
) -> Result<(StatusCode, Json<Value>), AppError>{
    register_new_user(&state.db, message_data).await
}