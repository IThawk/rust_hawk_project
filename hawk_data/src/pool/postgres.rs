/// A type alias for using `deadpool::Pool` with `redis`
pub type Pool = deadpool::managed::Pool<Self, ()>;

/// The manager for creating and recyling lapin connections
pub struct Manager {
    client: Client,
    params: String,
}

impl Manager {
    /// Create manager using `PgConfig` and a `TlsConnector`
    pub fn new(params: String) -> Result<Self, ()> {
        Ok(Self {
            client: Client::connect(params.as_str(), NoTls).expect("222"),
            params,
        })
    }
}

#[async_trait]
impl deadpool::managed::Manager<Self, ()> for Manager {
    async fn create(&self) -> Result<Client, ()> {
        let conn = Client::connect(self.params.as_str(), NoTls).expect("1222");
        Ok(conn)
    }

    async fn recycle(&self, conn: &mut Client) -> RecycleResult<String> {
        match conn.execute(
            "UPDATE foo SET bar = $1 WHERE baz = $2",
            &[&bar, &baz],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}


#[cfg(feature = "config")]
use ::config_crate::{ConfigError, Environment};
use deadpool::managed::{PoolConfig, RecycleResult};

use crate::{Pool, RedisResult};
// use redis::IntoConnectionInfo;
use postgres::{Client, NoTls};

/// Configuration object. By enabling the `config` feature you can
/// read the configuration using the [`config`](https://crates.io/crates/config)
/// crate.
/// ## Example environment
/// ```env
/// REDIS_URL=pg.example.com
/// REDIS_POOL.MAX_SIZE=16
/// REDIS_POOL.TIMEOUTS.WAIT.SECS=2
/// REDIS_POOL.TIMEOUTS.WAIT.NANOS=0
/// ```
/// ## Example usage
/// ```rust,ignore
/// Config::from_env("REDIS");
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "config", derive(serde::Deserialize))]
pub struct PostgresConfig {
    /// Redis URL
    /// See https://docs.rs/redis/0.15.1/redis/#connection-parameters
    pub url: Option<String>,
    /// Pool configuration
    pub pool: Option<PoolConfig>,
}

impl PostgresConfig {
    /// Create configuration from environment variables.
    #[cfg(feature = "config")]
    pub fn from_env(prefix: &str) -> Result<Self, ConfigError> {
        let mut cfg = ::config_crate::Config::new();
        cfg.merge(Environment::with_prefix(prefix))?;
        cfg.try_into()
    }
    /// Create pool using the current configuration
    pub fn create_pool(&self) -> Result<Pool,()> {
        let url = self.get_url();
        let manager = Manager::new(url.to_string())?;
        let pool_config = self.get_pool_config();
        Ok(Pool::from_config(manager, pool_config))
    }
    /// Get `URL` which can be used to connect to
    /// the database.
    pub fn get_url(&self) -> &str {
        if let Some(url) = &self.url {
            url
        } else {
            "redis://127.0.0.1/"
        }
    }
    /// Get `deadpool::PoolConfig` which can be used to construct a
    /// `deadpool::managed::Pool` instance.
    pub fn get_pool_config(&self) -> PoolConfig {
        self.pool.clone().unwrap_or_default()
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            url: None,
            pool: None,
        }
    }
}
