use std::net::SocketAddr;

use chrono::DateTime;
use tokio::{io::AsyncReadExt, net::TcpStream};

async fn request_handle(mut tcp_stream: TcpStream, _address: SocketAddr) -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(2048);

    tcp_stream.read_buf(&mut buf).await.unwrap();

    let (seconds, command) = protocol::decode_v1(&buf);

    // TODO add proper error handling
    let time = DateTime::from_timestamp(seconds as i64, 0).unwrap();
    tracing::info!(
        "Timestamp: {}, Command: {:?}",
        time.format("%d/%m/%Y %H:%M:%S"),
        command
    );

    Ok(())
}

pub async fn run_collection(bind_address: &str) -> anyhow::Result<()> {
    let handler = tokio::net::TcpListener::bind(bind_address).await?;

    loop {
        if let Ok((stream, address)) = handler.accept().await {
            let _ = tokio::spawn(request_handle(stream, address)).await?;
        } else {
            tracing::error!("Can't accept new connection...")
        }
    }
}
