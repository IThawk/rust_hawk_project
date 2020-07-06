use hawk_api::config_center::service_config::{ServiceConfig, ServicePosition};
use hawk_api::config_center::services_config::ServicesConfig;
use hawk_api::model::config::Config;
use hawk_api::model::redis::RedisConfig;
use hawk_api::CONNECTION_CHAR;
use hawk_tools::utils::yml_utils::read_yml;

///read server config from yaml file
pub fn read_server_config(config_path: String) -> Option<ServiceConfig> {
    match read_yml::<ServiceConfig>(config_path.as_str()) {
        Ok(config) => Some(config),
        Err(e) => {
            info!("read server config error");
            None
        }
    }
}

///read servers config from yaml file
/// if error make default ServicesConfig
pub fn read_servers_config(config_path: String) -> ServicesConfig {
    match read_yml::<ServicesConfig>(config_path.as_str()) {
        Ok(config) => config,
        Err(e) => {
            info!("read servers config error{}", e);
            ServicesConfig::default()
        }
    }
}

///find server with request path and heard projectId
pub fn find_server_by_req_path(
    servers_config: &ServicesConfig,
    req_path: &str,
    project: &str,
) -> Vec<String> {
    let mut re = Vec::new();

    for (server_c, o_req_path) in servers_config.paths.iter() {
        if o_req_path == req_path && server_c.contains(project) {
            re.push(server_c.clone());
        }
    }
    re
}

///find server positions in versions
pub fn find_server_port_by_req_path(
    servers_config: &ServicesConfig,
    servers: &Vec<String>,
    proto: &str,
    version: &str,
) -> Vec<ServicePosition> {
    let mut result = Vec::new();
    for q in servers.iter() {
        if q.contains(version) {
            if let Some(e) = q.rfind(CONNECTION_CHAR) {
                let b = &q[0..e];
                let target = format!("{}{}{}", b, CONNECTION_CHAR, proto);
                result.push(
                    servers_config
                        .positions
                        .get(target.as_str())
                        .unwrap_or(&ServicePosition::default())
                        .clone(),
                );
                continue;
            };

            warn!("server config has error :{}", q);
        }
    }
    result
}
