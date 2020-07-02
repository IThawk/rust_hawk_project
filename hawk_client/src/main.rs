#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate log;
#[macro_use]
extern crate tokio;
mod client;


use crate::client::{http, unix_socket};
use hawk_config::log_main;
use hawk_tools::utils::file_utils::read_file;
use hawk_tools::utils::os_utils;
use std::thread;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    // short and long flags (-p, --proto) will be deduced from the field's name
    #[structopt(short, long, default_value = "http")]
    pub proto: String,

    #[structopt(short = "u", long = "url", default_value = "http://httpbin.org/ip")]
    pub url: String,
}

/// hawk_client.exe -p https -u http:://127.0.0.1
fn main() {
    log_main();
    let opt: Opt = Opt::from_args();
    println!("{:?}", opt);
    info!("log init complete");

    let parked_thread_opt = thread::Builder::new().spawn(|| {
        unix_socket::main("http://httpbin.org/ip".to_string());
    });
    if parked_thread_opt.is_err() {
        println!("{:?}", parked_thread_opt);
        error!("thread run error : {:?}", parked_thread_opt);
    }
    //make a http server
    if opt.proto.as_str().eq("http") {
        http::main("http://httpbin.org/ip".to_string())
    }
}
