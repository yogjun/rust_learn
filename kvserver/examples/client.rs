use anyhow::Result;
use async_prost::AsyncProstStream;
use tokio::net::TcpStream;
use tracing::info;
use futures::prelude::*;
use kvserver::{CommandRequest, CommandResponse};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.0:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    /// AsyncProstStream
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    let cmd = CommandRequest::new_hset("table1","hello","world".into());

    // 发送hset
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await{
        info!("Got response {:?}",data);
    }
    Ok(())
}
