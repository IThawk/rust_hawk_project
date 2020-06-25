#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate log;
#[macro_use]
extern crate tokio;
mod client;
use crate::client::http;
use hawk_config::log_main;
use hawk_tools::utils::file_utils::read_file;
use hawk_tools::utils::os_utils;
use std::sync::Arc;
use std::thread;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {

    // short and long flags (-p, --proto) will be deduced from the field's name
    #[structopt(short, long, default_value = "http")]
    pub proto: String,

    #[structopt(short = "u", long = "url",default_value = "http://httpbin.org/ip")]
    pub url: String,
}


/// hawk_client.exe -p https -u http:://127.0.0.1
fn main() {
    log_main();
    let opt:Opt = Opt::from_args();
    println!("{:?}",opt);
    info!("log init complete");
    //TODO 启动一个线程
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            info!("Parking thread");
            info!("Thread unparked");
        })
        .unwrap();
    //make a http server
    if opt.proto.as_str().eq("http"){
        http::main("http://httpbin.org/ip".to_string());
    }

}
