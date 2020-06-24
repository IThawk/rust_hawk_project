use crate::model::default_ip_string;
///server config struct
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "ServerConfig::default_port_i32")]
    pub port: i32,
    #[serde(default = "ServerConfig::default_uds_string")]
    pub uds: String,
    #[serde(default = "ServerConfig::default_uds_open")]
    pub open_uds: bool,
}

impl ServerConfig {
    pub fn default_port_i32() -> i32 {
        8080
    }
    pub fn default_uds_string() -> String {
        "/usr/local/server.uds".to_string()
    }

    pub fn default_uds_open() -> bool {
        false
    }
}
