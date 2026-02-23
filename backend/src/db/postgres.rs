use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    tracing::info!("Database connection pool created");
    Ok(pool)
}

// Example query functions
pub async fn test_connection(pool: &PgPool) -> anyhow::Result<bool> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(1_i64)
        .fetch_one(pool)
        .await?;

    Ok(row.0 == 1)
}
