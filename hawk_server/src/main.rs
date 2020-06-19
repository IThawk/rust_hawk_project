#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate log;
extern crate hawk_api;
extern crate hawk_config;
mod config;
mod server;
mod utils;
use crate::config::config_center::ConfigCenter;
use config::log_main;
use hawk_config::utils::file_utils::read_file;
use server::http;
use std::sync::Arc;
use std::thread;

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
