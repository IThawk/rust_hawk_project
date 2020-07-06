use crate::config_center::service_config::ServiceConfig;
use crate::config_center::service_config::ServicePosition;
use crate::CONNECTION_CHAR;
use hawk_tools::utils::uuid_utils;
use std::collections::HashMap;

pub static FIRST_REVISION: &str = "0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub revision: String,
    #[serde(
        default = "HashMap::default",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub paths: HashMap<String, String>,
    #[serde(
        default = "HashMap::default",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub positions: HashMap<String, ServicePosition>,
}

impl ServicesConfig {
    pub fn register_server(&mut self, service_config: ServiceConfig) {
        if service_config.is_empty() {
            return;
        } else {
            //judge exist
            let service_str = format!(
                "{}{}{}{}{}",
                service_config.project,
                CONNECTION_CHAR,
                service_config.name,
                CONNECTION_CHAR,
                service_config.vision
            );
            if self.positions.contains_key(service_str.as_str()) {
                return;
            } else {
                for i_path in service_config.server_path {
                    let path = format!("{}{}{}", service_str, CONNECTION_CHAR, i_path);
                    self.paths.insert(path, i_path);
                }
                for i_position in service_config.server_positions {
                    let position_str =
                        format!("{}{}{}", service_str, CONNECTION_CHAR, i_position.proto);
                    self.positions.insert(position_str, i_position);
                }
                self.revision = uuid_utils::get_revision();
            }
        }
    }

    pub fn sign_out_server(&mut self, service_config: ServiceConfig) {
        if service_config.is_empty() {
            return;
        } else {
            //judge exist
            let service_str = format!(
                "{}{}{}{}{}",
                service_config.project,
                CONNECTION_CHAR,
                service_config.name,
                CONNECTION_CHAR,
                service_config.vision
            );
            if service_config.server_path.is_empty() && service_config.server_positions.is_empty() {
                return;
            }
            if !service_config.server_path.is_empty() {
                for i_path in service_config.server_path {
                    let path = format!("{}{}{}", service_str, CONNECTION_CHAR, i_path);
                    self.paths.remove(path.as_str());
                }
            }
            if !service_config.server_positions.is_empty() {
                for i_position in service_config.server_positions {
                    let position_str =
                        format!("{}{}{}", service_str, CONNECTION_CHAR, i_position.proto);
                    self.positions.remove(position_str.as_str());
                }
            }
            self.revision = uuid_utils::get_revision();
        }
    }
}

impl Default for ServicesConfig {
    fn default() -> Self {
        ServicesConfig {
            revision: FIRST_REVISION.to_string(),
            paths: HashMap::new(),
            positions: HashMap::new(),
        }
    }
}
