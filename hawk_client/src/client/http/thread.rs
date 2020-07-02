use actix_rt::System;
use hyper::{Body, Client, Uri};
use tokio::runtime::Runtime;

use hyper::client::{Builder, HttpConnector};
use std::str::FromStr;

pub fn make_http_client() -> Client<HttpConnector, Body> {
    Builder::default().build_http()
}

pub fn main(s: String) {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();
    // let mut rt = rt_opt.unwrap();
    rt.block_on(async move {
        let uri = Uri::from_str(s.as_str()).unwrap();
        let client = make_http_client();
        let res = client.get(uri).await.unwrap();
        println!("status: {}", res.status());
        let buf = hyper::body::to_bytes(res).await.unwrap();
        println!("body: {:?}", buf);
    })
}
