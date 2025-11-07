use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Pool, Result, Sqlite, sqlite::SqlitePool};
use uncovr::prelude::*;

pub async fn connect_db(url: &str) -> Result<Pool<Sqlite>> {
    SqlitePool::connect(url).await
}

pub async fn migrate(pool: &Pool<Sqlite>) -> Result<()> {
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            email       TEXT NOT NULL UNIQUE,
            password    TEXT NOT NULL,
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .await?;
    Ok(())
}

#[derive(Debug, FromRow)]
pub struct UserRecord {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(JsonSchema, Deserialize, Serialize, Default)]
pub struct CreateUser {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
}

#[derive(JsonSchema, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(JsonSchema, Deserialize, Serialize)]
pub struct TokenResponse {
    pub token: String,
}
