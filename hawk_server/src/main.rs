#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate log;
extern crate hawk_api;
extern crate hawk_config;
extern crate hawk_tools;

mod server;

//use crate::config::config_center::ConfigCenter;
use hawk_api::model::config::Config;
use hawk_config::log_main;
use hawk_tools::utils::file_utils::read_file;
use hawk_tools::utils::os_utils;
use server::http;
use server::unix_socket;
use std::sync::Arc;
use std::thread;

fn main() {
    //    make log by yml config
    //log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    //    let config = ConfigCenter::new();
    //    let config_arc = Arc::new(config);
    log_main();
    info!("log init complete");
    let unix_open = os_utils::is_windows();
    let config_option = hawk_config::read_config();
    let open_uds = open_uds(unix_open, config_option);
    //开启unixsocket 启动一个线程
    if open_uds {
        let parked_thread = thread::Builder::new().spawn(|| {
            unix_socket::main();
        });
        if parked_thread.is_err() {
            error!("start unix socket server error");
        }
    }

    //make a http server
    http::main("start".to_string());
}

fn open_uds(unix_open: bool, config_option: Option<Config>) -> bool {
    if unix_open {
        if let Some(config) = config_option {
            if let Some(server) = config.server {
                if server.open_uds {
                    return true;
                }
            }
        }
        false
    } else {
        false
    }
}
