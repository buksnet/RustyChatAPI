use crate::error::AppError;
use crate::models::{
    ChatCreationData, ChatEditData, ChatWithLastMessage, Message, MessageCreationData,
    MsgPaginatorQuery,
};
use axum::Json;
use axum::http::StatusCode;
use serde_json::json;
use sqlx::{PgPool, query, query_as};
use crate::utils::utils::is_moderator;

pub async fn get_chats_with_last_message(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<ChatWithLastMessage>, AppError> {
    let mut chats = query_as!(
        ChatWithLastMessage,
        r#"
        WITH user_chats AS (
            SELECT
                c.id AS chat_id,
                c.title,
                c.created_at AS chat_created_at
            FROM messenger.chats c
            JOIN messenger.chat_participants cp ON cp.chat_id = c.id
            WHERE cp.user_id = $1
        ),
        last_messages AS (
            SELECT DISTINCT ON (chat_id)
                chat_id,
                id AS message_id,
                sender_id,
                content,
                created_at AS message_created_at
            FROM messenger.messages
            WHERE chat_id IN (SELECT chat_id FROM user_chats)
            ORDER BY chat_id, created_at DESC
        )
        SELECT
            uc.chat_id,
            uc.title,
            u.name AS last_message_author,
            uc.chat_created_at,
            lm.content AS last_message,
            lm.message_created_at
        FROM user_chats uc
        LEFT JOIN last_messages lm ON lm.chat_id = uc.chat_id
        LEFT JOIN messenger.users u ON lm.sender_id = u.id
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("SQL error: {:?}", e);
        AppError::Database(e)
    })?;
    chats.sort_by(|a, b| {
        b.message_created_at
            .unwrap_or(b.chat_created_at)
            .cmp(&a.message_created_at.unwrap_or(a.chat_created_at))
    });
    Ok(chats)
}

pub async fn create_new_chat(
    pool: &PgPool,
    data: ChatCreationData,
    user_id: i32,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let new_chat = query!(
        r#"
        INSERT INTO messenger.chats(title)
        VALUES ($1)
        RETURNING id;
        "#,
        data.title
    )
    .fetch_one(pool)
    .await?;

    let chat_id = new_chat.id;

    for user_id in data.participants {
        query!(
            r#"
            INSERT INTO messenger.chat_participants(user_id, chat_id)
            VALUES ($1, $2)
            "#,
            user_id,
            chat_id
        )
        .execute(pool)
        .await?;
    }

    query!(
        r#"
    INSERT INTO messenger.chat_moderators(chat_id, moderator_id)
    VALUES ($1, $2);
    "#,
        chat_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok((StatusCode::CREATED, Json(json!({"status": "success"}))))
}

pub async fn edit_chat_data(
    pool: &PgPool,
    data: ChatEditData,
    chat_id: i32,
    user_id: i32,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {

    if is_moderator(&pool, &user_id, &chat_id).await? {
        query!(
            r#"
        UPDATE messenger.chats
        SET title = $2
        WHERE id = $1
        "#,
            chat_id,
            data.title
        )
        .execute(pool)
        .await?;
        // TODO: Добавить обновление изображения
        Ok((StatusCode::OK, Json(json!({"status": "success"}))))
    } else {
        Ok((
            StatusCode::FORBIDDEN,
            Json(json!({"status": "failed", "message": "insufficient permission to edit"})),
        ))
    }
}

pub async fn remove_chat(
    pool: &PgPool,
    chat_id: i32,
    user_id: i32,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {

    if is_moderator(&pool, &user_id, &chat_id).await? {
        query!(
            r#"
            DELETE FROM messenger.chats
            WHERE id = $1
            "#,
            chat_id
        )
        .execute(pool)
        .await?;
        Ok((StatusCode::OK, Json(json!({"status": "success"}))))
    } else {
        Ok((
            StatusCode::FORBIDDEN,
            Json(json!({"status": "failed", "message": "insufficient permission to edit"})),
        ))
    }
}

pub async fn create_new_message(
    pool: &PgPool,
    msg_data: MessageCreationData,
    user_id: i32,
) -> Result<i32, AppError> {
    let new_message_id: i32 = query!(
        r#"
        INSERT INTO messenger.messages(chat_id, sender_id, content)
        VALUES ($1, $2, $3)
        RETURNING chat_id;
        "#,
        msg_data.chat_id,
        user_id,
        msg_data.content
    )
    .fetch_one(pool)
    .await?
    .chat_id;
    Ok(new_message_id)
}

pub async fn get_messages_for_user(
    pool: &PgPool,
    paginator_req: MsgPaginatorQuery,
    user_id: i32,
) -> Result<Vec<Message>, AppError> {
    let messages = query_as!(
        Message,
        r#"
        SELECT
            m.id,
            m.chat_id,
            m.sender_id AS author_id,
            u.name AS author,
            m.content,
            m.created_at AS time_created,
            m.is_edited
        FROM messenger.messages m
        INNER JOIN messenger.chat_participants cp ON cp.chat_id = m.chat_id
        INNER JOIN messenger.users u ON u.id = m.sender_id
        WHERE m.chat_id = $1 AND cp.user_id = $2
        ORDER BY m.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        paginator_req.chat_id,
        user_id,
        paginator_req.limit,
        paginator_req.offset
    )
    .fetch_all(pool)
    .await?;
    Ok(messages)
}
