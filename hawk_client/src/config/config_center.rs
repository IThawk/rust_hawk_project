use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigCenter {
    pub redis_config: RedisConfig,
}

impl ConfigCenter {
    pub fn new() -> ConfigCenter {
        ConfigCenter {
            redis_config: RedisConfig::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    pub ip: String,
    pub port: String,
    pub password: Option<String>,
}

impl RedisConfig {
    pub fn new() -> RedisConfig {
        RedisConfig {
            ip: String::default(),
            port: String::default(),
            password: None,
        }
    }
}
