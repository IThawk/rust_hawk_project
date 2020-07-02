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
pub struct PostgresConfig {
    #[serde(default = "default_ip_string")]
    pub ip: String,
    #[serde(default = "PostgresConfig::default_port_i32")]
    pub port: i32,

    #[serde(default = "PostgresConfig::default_user_string")]
    pub user: String,

    #[serde(default = "PostgresConfig::default_db_string")]
    pub db: String,

    #[serde(default = "PostgresConfig::default_passwd_string")]
    pub passwd: String,
}

impl PostgresConfig {
    pub fn default_port_i32() -> i32 {
        6319
    }
    pub fn default_user_string() -> String {
        "".to_string()
    }
    pub fn default_db_string() -> String {
        "".to_string()
    }
    pub fn default_passwd_string() -> String {
        "".to_string()
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        PostgresConfig {
            ip: "127.0.0.1".to_string(),
            port: 5321,
            user: "postgres".to_string(),
            db: "postgres".to_string(),
            passwd: "123456".to_string(),
        }
    }
}
