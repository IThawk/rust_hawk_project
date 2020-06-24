#[macro_use]
extern crate log;
extern crate hawk_api;
extern crate hawk_tools;

pub mod log_config;

use crate::log_config::logger_main;
use hawk_api::model::config::Config;
use hawk_api::model::redis::RedisConfig;
use hawk_tools::utils::file_utils::{read_file, write_file};
use log::LevelFilter;

//pub mod utils;

pub fn read_config() -> Option<Config> {
    let a = read_file("./config/my_config.yml".to_string());
    if let Ok(context) = a {
        println!("{}", context);
        info!("{}", context);
        if let Ok(config) = serde_yaml::from_str::<Config>(&context) {
            println!("{:?}", config);
            info!("{:?}", config);
            return Some(config);
        };
    }
    info!("there is no config");
    return None;
}

pub fn log_main() {
    let (level, path) = init_log_level();
    if !path.is_empty() {
        logger_main(level, path.as_str())
    }
}

fn init_log_level() -> (LevelFilter, String) {
    if let Some(config) = read_config() {
        if let Some(log) = config.log {
            let path = log.path;
            if log.level == "trace" {
                return (LevelFilter::Trace, path);
            } else if log.level == "debug" {
                return (LevelFilter::Debug, path);
            } else if log.level == "warn" {
                return (LevelFilter::Warn, path);
            } else if log.level == "error" {
                return (LevelFilter::Error, path);
            } else if log.level == "error" {
                return (LevelFilter::Off, "".to_string());
            } else {
                return (LevelFilter::Info, path);
            }
        }
    }
    (LevelFilter::Info, "./log/request.log".to_string())
}
