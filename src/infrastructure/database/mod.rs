pub mod user_repository;

use sqlx::{Pool, Postgres};

pub type DatabasePool = Pool<Postgres>;

pub async fn create_database_pool(database_url: &str) -> crate::Result<DatabasePool> {
    let pool = sqlx::PgPool::connect(database_url).await?;
    
    // Create the users table if it doesn't exist
    sqlx::query(
        r#"
        CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
        "#
    )
    .execute(&pool)
    .await
    .map_err(|e| format!("Database setup error: {}", e))?;

    Ok(pool)
}