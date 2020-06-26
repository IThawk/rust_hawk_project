use actix_rt::System;
use hyper::{Body, Client, Uri, Request};
use tokio::runtime::Runtime;

use hyper::client::{Builder, HttpConnector};
use std::str::FromStr;
use hyper::client::connect::Connect;
// use hyper::client::connect::sealed::Internal;
use hyper::client::connect::dns::{GaiResolver};
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, Ipv6Addr, SocketAddrV6};
use std::sync::Arc;

use std::{future::Future, net::SocketAddr, pin::Pin, task::{self, Poll}};
use hyper::{service::Service};
use tokio::net::TcpStream;
use std::path::{Path, PathBuf};
use futures::task::Context;
// use hyper::client::connect::sealed::Internal;


pub fn main(s: String) {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();
        // let mut rt = rt_opt.unwrap();
        rt.block_on( async move{
            let uri = Uri::from_str(s.as_str()).unwrap();
            let client = make_uds_client();
            let res = client.get(uri).await.unwrap();
            println!("status: {}", res.status());
            let buf = hyper::body::to_bytes(res).await.unwrap();
            println!("body: {:?}", buf);
        })

}

pub fn make_uds_client() -> Client<LocalConnector, Body> {

    let connector = LocalConnector{};
    let a = Builder::default().build::<LocalConnector, Body>(connector);
    a
}

#[cfg(unix)]
impl Service<Uri> for LocalConnector {
    type Response = tokio::net::UnixStream;
    type Error = std::io::Error;
    // We can't "name" an `async` generated future.
    type Future = Pin<Box<
        dyn Future<Output = Result<Self::Response, Self::Error>> + Send
    >>;

    fn poll_ready(&mut self, _: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        // This connector is always ready, but others might not be.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, dst: Uri) -> Self::Future {
        println!("{:?}",dst);
        println!("{:?}",dst.host());
        println!("{:?}",dst.port_u16());
        let p = PathBuf::from("/test/test.sock");
        let c = p.as_path();
        Box::pin(tokio::net::UnixStream::connect(c))
    }
}




#[derive(Clone)]
pub struct LocalConnector;

#[cfg(windows)]
impl Service<Uri>  for LocalConnector{
    type Response = tokio::net::TcpStream;
        type Error = std::io::Error;
    // We can't "name" an `async` generated future.
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, dst: Uri) -> Self::Future {
                let mut addrs =
            try_parse(dst.host().expect("host"), dst.port_u16().unwrap_or(80))
                .expect("try_parse");
        Box::pin(tokio::net::TcpStream::connect(addrs))
    }
}


pub fn try_parse(host: &str, port: u16) -> Option<SocketAddr> {
    if let Ok(addr) = host.parse::<Ipv4Addr>() {
        let addr = SocketAddrV4::new(addr, port);

        // let socket = SocketAddr::new(SocketAddrV4::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        return Some(SocketAddr::from(addr));
    }
    let host = host.trim_start_matches('[').trim_end_matches(']');
    if let Ok(addr) = host.parse::<Ipv6Addr>() {
        let addr = SocketAddrV6::new(addr, port, 0, 0);
        Some(SocketAddr::from(addr));
    }
    None
}

