mod handlers; mod models; mod error; mod db;

use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;
use sqlx::PgPool;


#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::create_pool().await?;
    let app = Router::new()
        .route("/messages", get(handlers::get_messages))
        .with_state(AppState { db: pool });

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;
    Ok(())
}