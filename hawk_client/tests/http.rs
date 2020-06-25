use hyper::client::{Builder, HttpConnector};
use hyper::{Body, Client, Uri};
use std::io;

pub fn make_http_client() -> Client<HttpConnector, Body> {
    Builder::default().build_http()
}

/// test connect redis and test pipeline
///
#[tokio::main]
#[test]
async fn test_pipeline() {
    let client = make_http_client();
    let res = client
        .get(Uri::from_static("http://httpbin.org/ip"))
        .await
        .unwrap();
    println!("status: {}", res.status());
    let buf = hyper::body::to_bytes(res).await.unwrap();
    println!("body: {:?}", buf);
}
