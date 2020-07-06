//use crate::config::config_center::ConfigCenter;
use crate::hawk_api::model::redis::CacheInfo;

use futures_util::future::try_join;

use std::error::Error;
use std::net::SocketAddr;
use tokio::io;
use tokio::net::TcpStream;
pub async fn http_proxy(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = io::copy(&mut ri, &mut wo);
    let server_to_client = io::copy(&mut ro, &mut wi);

    try_join(client_to_server, server_to_client).await?;

    Ok(())
}
