use deadpool::managed::{PoolConfig, Timeouts};
use deadpool_redis::{cmd, Config, Pool, PoolError};
use hawk_api::model::redis::RedisConfig;
use hawk_api::traits::Connection;
use hawk_data::db::DbConnection;
use redis::FromRedisValue;
use std::io;
/// test connect redis and test pipeline
///
#[tokio::main]
#[test]
async fn test_pipeline() {
    use deadpool_redis::{pipe, Config};
    let redis_config = RedisConfig::default();
    if let Ok(pool) = DbConnection::<RedisConfig, Pool>::connect_to_db(redis_config) {
        if let Ok(mut conn) = pool.get().await {
            let (value,): (String,) = pipe()
                .cmd("SET")
                .arg("deadpool/pipeline_test_key")
                .arg("42")
                .ignore()
                .cmd("GET")
                .arg("deadpool/pipeline_test_key")
                .query_async(&mut conn)
                .await
                .unwrap();
            println!("{}", value);
            assert_eq!(value, "42".to_string());
        }
    };
}
