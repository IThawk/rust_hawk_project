use crate::model::default_ip_string;
///mysql config struct
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlConfig {
    #[serde(default = "default_ip_string")]
    pub ip: String,
    #[serde(default = "MysqlConfig::default_port_i32")]
    pub port: i32,

    pub passwd: String,
}

impl MysqlConfig {
    pub fn default_port_i32() -> i32 {
        3306
    }
}
