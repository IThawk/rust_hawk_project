extern crate hawk_client;
use futures_util::stream::TryStreamExt;
#[cfg(unix)]
use hawk_client::uds::client::UnixClientExt;
#[cfg(unix)]
use hawk_client::uds::uri::Uri as UnixUri;
use hyper::{Client, Uri};
use std::error::Error;
use std::str::FromStr;

#[cfg(unix)]
#[tokio::main]
#[test]
async fn test_uds() {
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
