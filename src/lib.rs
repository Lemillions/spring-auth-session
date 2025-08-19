pub mod config;
pub mod domain;
pub mod errors;
pub mod infrastructure;

use crate::config::Config;
use crate::infrastructure::database::{create_database_pool, DatabasePool};
use crate::infrastructure::redis::{create_redis_pool, RedisPool};
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: DatabasePool,
    pub redis_pool: RedisPool,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        let db_pool = create_database_pool(&config.database_url).await?;
        let redis_pool = create_redis_pool(&config.redis_url).await?;

        Ok(Self {
            db_pool,
            redis_pool,
            config: Arc::new(config),
        })
    }
}