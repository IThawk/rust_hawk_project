use crate::model::default_ip_string;
use hawk_tools::utils::os_utils;
///client config struct
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig {
    #[serde(default = "default_ip_string")]
    pub ip: String,
    #[serde(default = "ClientConfig::default_port_i32")]
    pub port: i32,
    #[serde(default = "ClientConfig::default_uds_string")]
    pub uds: String,
    #[serde(default = "ClientConfig::default_uds_open")]
    pub open_uds: bool,
}

impl ClientConfig {
    pub fn default_port_i32() -> i32 {
        8080
    }

    pub fn default_uds_string() -> String {
        "/usr/local/server.uds".to_string()
    }

    pub fn default_uds_open() -> bool {
        false
    }

    pub fn is_use_uds(&self)->Option<String>{
        if os_utils::is_windows(){
            return None
        }
        if self.open_uds{
            if !self.uds.is_empty(){
                return Some(self.uds.clone())
            }
        }
        return None
    }
}
