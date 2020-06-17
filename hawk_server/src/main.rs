#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate log;
extern crate hawk_config;
extern crate hawk_api;
mod server;
mod config;
mod utils;
use config::log_main;
use server::http;
use std::thread;
use crate::config::config_center::ConfigCenter;
use std::sync::Arc;
use hawk_config::utils::file_utils::read_file;


fn main() {
//    make log by yml config
    //log4rs::init_file("log4rs.yml", Default::default()).unwrap();
//    let config = ConfigCenter::new();
//    let config_arc = Arc::new(config);
    log_main();
    info!("log init complete");
    //TODO 启动一个线程
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            info!("Parking thread");
            info!("Thread unparked");
        })
        .unwrap();
    //make a http server
    http::main();
}
