use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result1<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn main(url:String) {
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
        // return Ok(());
    }

    fetch_url(url).await;

}

async fn fetch_url(url: hyper::Uri) -> Result1<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    // asynchronously aggregate the chunks of the body
    let body = hyper::body::to_bytes(res).await?;
    // let users = serde_json::from_reader(body.reader())?;
    println!("Response: {}", body[1]);
    Ok(())
}
