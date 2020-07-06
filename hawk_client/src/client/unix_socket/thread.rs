extern crate hawk_client;
use futures_util::stream::TryStreamExt;
#[cfg(unix)]
use hawk_client::uds::client::UnixClientExt;
#[cfg(unix)]
use hawk_client::uds::uri::Uri as UnixUri;
use hyper::{Client, Uri};
use std::error::Error;
use std::str::FromStr;
use tokio::runtime::Runtime;

// #[tokio::main]
pub fn main(s: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut rt = Runtime::new().expect("runtime expect!");
    rt.block_on(async move {
        #[cfg(windows)]
        {
            let client = Client::new();
            if let Ok(uri) = Uri::from_str(s.as_str()) {
                // Make a GET /ip to 'http://httpbin.org'
                if let Ok(res) = client.get(uri).await {
                    // And then, if the request gets a response...
                    println!("status: {}", res.status());
                    // Concatenate the body stream into a single buffer...
                    if let Ok(buf) = hyper::body::to_bytes(res).await {
                        println!("body: {:?}", buf);
                    };
                };
            };
        }

        #[cfg(unix)]
        {
            let url = UnixUri::new("/tmp/hyperlocal.sock", "/").into();
            let client = Client::unix();
            if let Ok(response_body) = client.get(url).await {
                if let Ok(bytes) = response_body
                    .into_body()
                    .try_fold(Vec::default(), |mut v, bytes| async {
                        v.extend(bytes);
                        Ok(v)
                    })
                    .await
                {
                    println!("{}", String::from_utf8(bytes).unwrap());
                };
            };
        }
    });

    Ok(())
}
