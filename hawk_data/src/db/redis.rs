use crate::db::DbConnection;
use deadpool::managed::{PoolConfig, Timeouts};
use deadpool_redis::{cmd, Config, Pool};
use hawk_api::model::redis::RedisConfig;
use hawk_api::traits::Connection;
use redis::FromRedisValue;

impl Connection<RedisConfig, Pool> for DbConnection<RedisConfig, Pool> {
    fn connect_to_db(connection_config: RedisConfig) -> Result<Pool, String> {
        let redis_url = if connection_config.passwd.is_empty() {
            format!(
                "redis://{}:{}",
                connection_config.ip, connection_config.port
            )
        } else {
            format!(
                "redis://{}@{}:{}",
                connection_config.passwd, connection_config.ip, connection_config.port
            )
        };
        let pool_config = PoolConfig {
            max_size: 10,
            timeouts: Timeouts::new(),
        };
        let cfg = Config {
            url: Some(redis_url.to_string()),
            /// Pool configuration
            pool: Some(pool_config),
        };
        if let Ok(pool) = cfg.create_pool() {
            return Ok(pool);
        }
        Err("connection redis error".to_string())
    }
}

impl Connection<RedisConfig, Config> for DbConnection<RedisConfig, Config> {
    fn connect_to_db(connection_config: RedisConfig) -> Result<Config, String> {
        unimplemented!()
    }
}

// #[tokio::test]
async fn test_managed_basic() {
    let redis_config = RedisConfig::default();
    let pool = DbConnection::<RedisConfig, Pool>::connect_to_db(redis_config);
    // println!("{:?}", c)
    if let Ok(pool) = pool {
        let mut conn = pool.get().await.unwrap();
        cmd("SET")
            .arg(&["deadpool/test_key", "42"])
            .execute_async(&mut conn)
            .await
            .unwrap();
        let mut conn = pool.get().await.unwrap();
        let value: String = cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async(&mut conn)
            .await
            .unwrap();
        assert_eq!(value, "42".to_string());
    }
}
