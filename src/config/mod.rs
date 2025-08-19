use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> crate::Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        let database_url = env::var("DATABASE_URL")
            .or_else(|_: env::VarError| {
                let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
                let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
                let db_name = env::var("DB_NAME").unwrap_or_else(|_| "session_auth".to_string());
                let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
                let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
                Ok(format!(
                    "postgres://{}:{}@{}:{}/{}",
                    db_user, db_password, db_host, db_port, db_name
                ))
            })
            .map_err(|_: env::VarError| "DATABASE_URL or DB_* environment variables must be set")?;

        let redis_url = env::var("REDIS_URL")
            .or_else(|_: env::VarError| {
                let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
                let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
                Ok(format!("redis://{}:{}", redis_host, redis_port))
            })
            .map_err(|_: env::VarError| "REDIS_URL or REDIS_* environment variables must be set")?;

        let server_port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "PORT must be a valid number")?;

        Ok(Self {
            database_url,
            redis_url,
            server_port,
        })
    }
}