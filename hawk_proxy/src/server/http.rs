use crate::http::thread::http_proxy;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;

use futures::future::try_join;
use futures::FutureExt;
use std::env;
use std::error::Error;
pub fn main(s: String) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
    let client = HttpClient::new();

    // Create the runtime
    let mut rt = Runtime::new().unwrap();
    // let mut rt = rt_opt.unwrap();
    rt.block_on(async move {
        let listen_addr = env::args()
            .nth(1)
            .unwrap_or_else(|| "127.0.0.1:8089".to_string());
        let server_addr = env::args()
            .nth(2)
            .unwrap_or_else(|| "127.0.0.1:1337".to_string());

        println!("Listening on: {}", listen_addr);
        println!("Proxying to: {}", server_addr);

        let mut listener = TcpListener::bind(listen_addr).await.unwrap();

        while let Ok((inbound, _)) = listener.accept().await {
            let transfer = http_proxy(inbound, server_addr.clone()).map(|r| {
                if let Err(e) = r {
                    println!("Failed to transfer; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
    });
}
