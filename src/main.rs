mod handlers;
mod repositories;
mod models;
mod error;
mod db;
mod utils;


use axum::{routing::get, serve, Router};
use axum::routing::{get_service, post};
use tokio::net::TcpListener;
use sqlx::PgPool;
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};


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

    sqlx::migrate!().run(&pool).await?;

    let app = Router::new()
        .layer(TraceLayer::new_for_http())

        .route("/chats", get(handlers::get_chats))
        .route("/chats", post(handlers::new_chat))

        .route("/messages", get(handlers::fetch_messages))
        .route("/messages", post(handlers::new_message))

        .route("/auth/login", post(handlers::login))
        .route("/auth/register", post(handlers::register))

        .route("/app/download", get_service(ServeFile::new("static/rustychat.apk")))

        .with_state(AppState { db: pool });

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;
    Ok(())
}
