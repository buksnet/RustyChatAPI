use axum::{response::IntoResponse, http::StatusCode};

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("AppError: {:?}", self);
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            _ => StatusCode::CONFLICT.into_response()
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self { AppError::Database(err) }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AppError::InternalServerError
    }
}
