use std::net::SocketAddr;

use chrono::DateTime;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn request_handle(mut tcp_stream: TcpStream, _address: SocketAddr) {
    let mut buf = Vec::with_capacity(2048);

    tcp_stream.read_buf(&mut buf).await.unwrap();

    let (seconds, command) = protocol::decode_v1(&buf);

    // TODO add proper error handling
    let time = DateTime::from_timestamp(seconds as i64, 0).unwrap();
    println!(
        "Timestamp: {}, Command: {:?}",
        time.format("%d/%m/%Y %H:%M:%S"),
        command
    );
}
