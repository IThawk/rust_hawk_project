#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Redis {
    #[serde(default = "default_ip_string")]
    pub ip:String,
    pub port: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Mysql {
    #[serde(default = "default_ip_string")]
    pub ip:String,
    pub port: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(default = "default_server_port_i32")]
    pub port:i32,
    #[serde(default = "default_server_uds_string")]
    pub uds: String,
    #[serde(default = "default_server_uds_default")]
    pub open_uds : bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_log_level_string")]
    pub level:String,
    #[serde(default = "default_log_path_string")]
    pub path: String,

}

///read https://serde.rs/attr-skip-serializing.html
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<Redis>,
    //skip serialize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql: Option<Mysql>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<LogConfig>,
}


fn default_ip_string() -> String {
    "127.0.0.1".to_string()
}

fn default_server_port_i32() -> i32 {
    8080
}

fn default_server_uds_string() -> String {
    "/usr/local/server.uds".to_string()
}

fn default_server_uds_default() -> bool {
    false
}

fn default_log_level_string() -> String {
    "Warn".to_string()
}


fn default_log_path_string() -> String {
    "./log/request.log".to_string()
}
