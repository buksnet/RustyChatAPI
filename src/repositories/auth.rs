use crate::error::AppError;
use crate::models::{Claims, LoginData, RegisterData};
use crate::utils::tag_generator::TagGenerator;
use crate::utils::utils::{hash_string, verify_password};
use axum::Json;
use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header, encode};
use regex::RegexBuilder;
use serde_json::{Value, json};
use sqlx::{PgPool, query};

pub async fn get_new_token(
    pool: &PgPool,
    user_data: LoginData,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let regex = RegexBuilder::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
        .build()
        .unwrap();

    // Определяем, ищем по email или по tag
    let (usr_id, usr_pwd_hash) = if regex.is_match(&user_data.username) {
        let usr = query!(
            "SELECT id, pwd_hash FROM messenger.users WHERE email = $1",
            &user_data.username
        )
            .fetch_optional(pool)
            .await?;

        match usr {
            Some(user) => (user.id, user.pwd_hash),
            None => return Ok((StatusCode::NOT_FOUND, Json(json!({"status": "user not found"})))),
        }
    } else {
        let usr = query!(
            "SELECT id, pwd_hash FROM messenger.users WHERE tag = $1",
            &user_data.username
        )
            .fetch_optional(pool)
            .await?;

        match usr {
            Some(user) => (user.id, user.pwd_hash),
            None => return Ok((StatusCode::NOT_FOUND, Json(json!({"status": "user not found"})))),
        }
    };

    // Проверяем пароль
    if !verify_password(&usr_pwd_hash, &user_data.password)? {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(json!({"status": "error", "description": "wrong credentials"})),
        ));
    }

    // Генерируем токен
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: usr_id,
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(json!({"token": token}))))
}

pub async fn register_new_user(
    pool: &PgPool,
    user_data: RegisterData,
) -> Result<(StatusCode, Json<Value>), AppError> {
    // Проверяем, не занят ли email
    let email_exists = query!(
        "SELECT id FROM messenger.users WHERE email = $1",
        user_data.email
    )
        .fetch_optional(pool)
        .await?
        .is_some();

    if email_exists {
        return Ok((
            StatusCode::CONFLICT,
            Json(json!({"status": "error", "field": "email", "message": "email already exists"})),
        ));
    }

    // Если tag передан — проверяем, не занят ли он
    if let Some(tag) = &user_data.tag {
        let tag_exists = query!(
            "SELECT id FROM messenger.users WHERE tag = $1",
            tag
        )
            .fetch_optional(pool)
            .await?
            .is_some();

        if tag_exists {
            return Ok((
                StatusCode::CONFLICT,
                Json(json!({"status": "error", "field": "tag", "message": "tag already taken"})),
            ));
        }
    }

    // Валидация пароля
    if user_data.password.len() < 8 {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "error", "field": "password", "message": "password too short, minimum 8 characters"})),
        ));
        // TODO: Проверка сложности пароля
    }

    let password_hash = hash_string(&user_data.password).await?;

    let mut tag_gen = TagGenerator::new();
    let alt_tag= tag_gen.get_nick() ;

    let tag = user_data.tag.as_deref().unwrap_or(&alt_tag);



    let usr = query!(
        r#"
        INSERT INTO messenger.users (name, email, tag, pwd_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, pwd_hash
        "#,
        &user_data.username,
        &user_data.email,
        &tag,
        &password_hash
    )
    .fetch_one(pool)
    .await?;

    tag_gen.clear(); // очистка памяти от списка слов

    // Генерируем токен
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: usr.id,
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
        .map_err(|_| AppError::InternalServerError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({"status": "success", "id": usr.id, "token": token})),
    ))
}
