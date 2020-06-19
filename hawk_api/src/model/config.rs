#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Redis {
    pub port: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Mysql {
    pub port: i32,
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
}
