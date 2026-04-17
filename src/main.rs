mod handlers;
mod repositories;
mod models;
mod error;
mod db;
mod utils;
mod auth;

use axum::{middleware, routing::get, serve, Router};
use axum::routing::{get_service, post};
use tokio::net::TcpListener;
use sqlx::PgPool;
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::auth::auth_middleware;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    let pool = db::create_pool().await?;

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    sqlx::migrate!().run(&pool).await?;

    let protected_routes = Router::new()
        .layer(TraceLayer::new_for_http())

        .route("/chats", get(handlers::get_chats))
        .route("/chats", post(handlers::new_chat))

        .route("/messages", get(handlers::fetch_messages))
        .route("/messages", post(handlers::new_message))
        .layer(middleware::from_fn_with_state(secret.clone(), auth_middleware));



    let all_routes = Router::new()
        .route("/auth/login", post(handlers::login))
        .route("/auth/register", post(handlers::register))
        .route("/app/download", get_service(ServeFile::new("static/rustychat.apk")))
        .merge(protected_routes)
        .with_state(AppState { db: pool });

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, all_routes).await?;
    Ok(())
}
