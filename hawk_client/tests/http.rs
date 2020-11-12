use hyper::client::{Builder, HttpConnector};
use hyper::{Body, Uri};
use std::io as io1;
pub fn make_http_client() -> Client<HttpConnector, Body> {
    Builder::default().build_http()
}

/// test connect redis and test pipeline
///
#[tokio::main]
#[test]
async fn test_http() {
    let client = make_http_client();
    let res = client
        .get(Uri::from_static("http://httpbin.org/ip"))
        .await
        .unwrap();
    println!("status: {}", res.status());
    let buf = hyper::body::to_bytes(res).await.unwrap();
    println!("body: {:?}", buf);
}

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

#[tokio::main]
#[test]
async fn main() -> std::result::Result<(), reqwest::Error> {
    let res = reqwest::get("http://127.0.0.1:8099/kkkl").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}


use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result1<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
#[test]
async fn main1() -> Result1<()> {
    // pretty_env_logger::init();

    // Some simple CLI args requirements...
    // let url = match env::args().nth(1) {
    //     Some(url) => url,
    //     None => {
    //         println!("Usage: client <url>");
    //         return Ok(());
    //     }
    // };

    let url = "http://127.0.0.1:8099/kkkl";

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result1<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}