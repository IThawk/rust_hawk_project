use crate::model::client::ClientConfig;
use crate::model::log::LogConfig;
use crate::model::mysql::MysqlConfig;
use crate::model::redis::RedisConfig;
use crate::model::server::ServerConfig;

///totalConfig
/// read https://serde.rs/attr-skip-serializing.html
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<RedisConfig>,
    //skip serialize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql: Option<MysqlConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<LogConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientConfig>,
}
