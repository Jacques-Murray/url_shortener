use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::env;

use crate::models::UrlRecord;

pub async fn create_db_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub async fn find_url_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<UrlRecord, sqlx::Error> {
    sqlx::query_as::<_, UrlRecord>("SELECT original_url FROM urls WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn save_url(
    pool: &Pool<Sqlite>,
    id: &str,
    original_url: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO urls (id, original_url) VALUES (?, ?)")
        .bind(id)
        .bind(original_url)
        .execute(pool)
        .await?;
    Ok(())
}
