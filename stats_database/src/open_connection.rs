use dotenvy::dotenv;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;

pub async fn create_pool() -> sqlx::Result<Pool<Sqlite>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_players() {
        let pool = create_pool().await;
        assert!(pool.is_ok())
    }

}
