use crate::error;
use dotenvy::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

pub async fn create_pool() -> error::Result<Pool<Sqlite>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;
    let manager = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url);

    let pool = manager.await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_pool().await;
        assert!(pool.is_ok())
    }
}
