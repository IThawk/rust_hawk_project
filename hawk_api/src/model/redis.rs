use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct CacheInfo {
    pub one: String,
    pub two: String,
    pub three: String,
}