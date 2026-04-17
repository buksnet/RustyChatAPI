use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use crate::models::Claims;

pub async fn auth_middleware(
    State(secret): State<String>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    // Достаём токен из заголовка Authorization
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"status": "error", "message": "missing or invalid authorization header"})),
            )
                .into_response());
        }
    };

    // Декодируем и проверяем токен
    let claims = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(c) => c.claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"status": "error", "message": "invalid or expired token"})),
            )
                .into_response());
        }
    };

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}