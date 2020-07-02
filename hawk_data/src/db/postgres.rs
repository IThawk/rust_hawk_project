use crate::db::DbConnection;
use deadpool::managed::{PoolConfig, Timeouts};
use deadpool_redis::{cmd, Config, Pool};
use hawk_api::model::postgres::PostgresConfig;
use hawk_api::traits::Connection;
use redis::FromRedisValue;

impl Connection<PostgresConfig, Pool> for DbConnection<PostgresConfig, Pool> {
    fn connect_to_db(connection_config: PostgresConfig) -> Result<Pool, String> {
        let postgres_url = if connection_config.passwd.is_empty() {
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
            url: Some(postgres_url.to_string()),
            /// Pool configuration
            pool: Some(pool_config),
        };
        if let Ok(pool) = cfg.create_pool() {
            return Ok(pool);
        }
        Err("connection redis error".to_string())
    }
}

impl Connection<PostgresConfig, Config> for DbConnection<PostgresConfig, Config> {
    fn connect_to_db(connection_config: PostgresConfig) -> Result<Config, String> {
        unimplemented!()
    }
}
