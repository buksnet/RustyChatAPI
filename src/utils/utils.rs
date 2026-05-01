use argon2::password_hash::Error as ArgonError;
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::{query, PgPool};

pub(crate) async fn hash_string(password: &str) -> Result<String, ArgonError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub(crate) fn verify_password(hash: &str, password: &str) -> Result<bool, ArgonError> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub async fn is_moderator(pool: &PgPool, user_id: &i32, chat_id: &i32) -> Result<bool, sqlx::Error> {
    let is_user_moderator = query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM messenger.chat_moderators
            WHERE messenger.chat_moderators.moderator_id = $1 AND messenger.chat_moderators.chat_id = $2
            ) AS exists
        "#,
        user_id,
        chat_id
    )
        .fetch_one(pool)
        .await?
        .exists;
    Ok(is_user_moderator.unwrap_or(false))
}