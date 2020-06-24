///logConfig config struct
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "LogConfig::default_level_string")]
    pub level: String,
    #[serde(default = "LogConfig::default_path_string")]
    pub path: String,
}

impl LogConfig {
    pub fn default_level_string() -> String {
        "Warn".to_string()
    }

    pub fn default_path_string() -> String {
        "./log/request.log".to_string()
    }
}
