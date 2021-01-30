extern crate serde_yaml;

use crate::utils::file_utils::{read_file, write_file};
use futures::Future;
use redis_async::client::PairedConnection;
use redis_async::{client, resp_array};
use redis_async::{error, resp};
use serde::de::DeserializeOwned;
use serde::export::Result::Ok;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

pub fn read_yml<T>(
    connection: &mut PairedConnection,
    common: &str,
    key: &str,
) -> impl Future<Output = Result<T, error::Error>>
where
    T: resp::FromResp,
{
    let incr_f = connection.send::<T>(resp_array![common, key]);
    incr_f
}

pub async fn get_connection<T>(
    connection: &mut PairedConnection,
    common: &str,
    key: &str,
) -> Result<PairedConnection, error::Error> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap()), 6379);
    let mut connection = client::paired_connect(&socket)
        .await
        .expect("Cannot open connection");
    Ok(connection)
}

#[actix_rt::test]
async fn redis_async_read_test() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1").unwrap()), 6379);
    let mut connection = client::paired_connect(&socket)
        .await
        .expect("Cannot open connection");
    let re = read_yml::<i32>(&mut connection, "INCR", "test");
    let a = re.await;
    match a {
        Ok(i) => println!("{}", i),
        Err(e) => println!("error"),
    }
}
