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
    let regex = RegexBuilder::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@a-zA-Z0-9?(?:.a-zA-Z0-9?)*$")
        .build()
        .unwrap();

    // если формат переданного username - email
    let is_user_data_correct: bool;
    let usr_id: i32;
    let usr_pwd_hash: String;
    if regex.is_match(&user_data.username) {
        let usr = query!(
            r#"
        SELECT id, pwd_hash FROM messenger.users
        WHERE email = $1
        "#,
            &user_data.username
        )
        .fetch_one(pool)
        .await?;
        usr_id = usr.id;
        usr_pwd_hash = usr.pwd_hash;
    } else {
        let usr = query!(
            r#"
        SELECT id, pwd_hash FROM messenger.users
        WHERE tag = $1
        "#,
            &user_data.username
        )
        .fetch_one(pool)
        .await?;
        usr_id = usr.id;
        usr_pwd_hash = usr.pwd_hash;
    }
    if verify_password(&usr_pwd_hash, &user_data.password)? {
        is_user_data_correct = true;
    } else {
        is_user_data_correct = false;
    }
    if is_user_data_correct {
        let secret = std::env::var("JWT_SECRET").expect("SECRET_KEY must be set");
        let claims = Claims {
            sub: usr_id, // надо достать id из запроса в БД
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();
        Ok((StatusCode::OK, Json(json!({"token": token}))))
    } else {
        Ok((
            StatusCode::FORBIDDEN,
            Json(json!({"status": "error", "description": "wrong credentials"})),
        ))
    }
}

pub async fn register_new_user(
    pool: &PgPool,
    user_data: RegisterData,
) -> Result<(StatusCode, Json<Value>), AppError> {
    // TODO: Валидация данных (в частности сложности пароля)
    let password_hash = hash_string(&user_data.password).await?;

    let mut tag_gen = TagGenerator::new();
    let alt_tag= tag_gen.get_nick() ;

    let tag = user_data.tag.as_deref().unwrap_or(&alt_tag);

    tag_gen.clear(); // очистка памяти от списка слов

    let usr = query!(
        r#"
        INSERT INTO messenger.users (name, email, tag, pwd_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, pwd_hash
        "#,
        &user_data.username,
        &user_data.email,
        tag,
        &password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({"status": "success", "id": usr.id})),
    ))
}
