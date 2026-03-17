use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenvy::dotenv;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL должен быть указан.");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}