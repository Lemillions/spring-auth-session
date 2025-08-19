pub mod session;

use redis::aio::MultiplexedConnection;
use redis::Client;

pub type RedisPool = MultiplexedConnection;

pub async fn create_redis_pool(redis_url: &str) -> crate::Result<RedisPool> {
    let client = Client::open(redis_url)?;
    let connection = client.get_multiplexed_async_connection().await?;
    Ok(connection)
}