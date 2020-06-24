use crate::model::default_ip_string;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct CacheInfo {
    pub one: String,
    pub two: String,
    pub three: String,
}

///redis config struct
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisConfig {
    #[serde(default = "default_ip_string")]
    pub ip: String,
    #[serde(default = "RedisConfig::default_port_i32")]
    pub port: i32,
    #[serde(
        default = "RedisConfig::default_uds_string",
        skip_serializing_if = "String::is_empty"
    )]
    pub unix_path: String,
}

impl RedisConfig {
    pub fn default_port_i32() -> i32 {
        6319
    }
    pub fn default_uds_string() -> String {
        "".to_string()
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            ip: "127.0.0.1".to_string(),
            port: 6379,
            unix_path: "".to_string(),
        }
    }
}
