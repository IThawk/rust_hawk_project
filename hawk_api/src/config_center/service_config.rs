use crate::traits::{Random, Round};
use hawk_tools::algorithm::random::random_int;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub project: String,
    pub name: String,
    pub vision: String,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub server_path: Vec<String>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub server_positions: Vec<ServicePosition>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServicePosition {
    pub proto: String,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<String>,
}

impl Default for ServicePosition {
    fn default() -> Self {
        ServicePosition {
            proto: String::default(),
            positions: Vec::default(),
        }
    }
}

impl ServiceConfig {
    pub fn is_empty(&self) -> bool {
        if self.server_positions.is_empty() {
            true
        } else {
            false
        }
    }
}

impl Random<ServicePosition> for ServicePosition {
    fn get_random_one(source: Vec<ServicePosition>) -> ServicePosition {
        if source.is_empty() {
            return ServicePosition::default();
        }
        let index = random_int((source.len() - 1) as i32);
        let a = source.get(index as usize);
        if let Some(s) = a {
            return s.clone();
        }
        return ServicePosition::default();
    }
}

impl Round<ServicePosition> for ServicePosition {
    fn get_random_one(source: Vec<ServicePosition>, before: i32) -> ServicePosition {
        if source.is_empty() {
            return ServicePosition::default();
        }
        let index = ((before + 1) as usize) % source.len();
        let a = source.get(index as usize);
        if let Some(s) = a {
            return s.clone();
        }
        return ServicePosition::default();
    }
}
