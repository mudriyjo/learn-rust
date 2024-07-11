use std::{net::SocketAddr};

use chrono::DateTime;
use protocol::{CollectorCommand, Commands};
use tokio::{io::AsyncReadExt, net::TcpStream};

fn print_command(seconds: &u32, command: &CollectorCommand) {
    let time = DateTime::from_timestamp(*seconds as i64, 0).unwrap();
    tracing::info!(
        "Timestamp: {}, Command: {:?}",
        time.format("%d/%m/%Y %H:%M:%S"),
        command
    );
}

async fn request_handle(mut tcp_stream: TcpStream, _address: SocketAddr) -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(2048);

    // TODO add parse many messages
    tcp_stream.read_buf(&mut buf).await?;

    // TODO add proper error handling
    match protocol::decode_v1(&buf) {
        Commands::Command((seconds, command)) => print_command(&seconds, &command),
        Commands::Commands(commands_list) => commands_list
            .into_iter()
            .for_each(|el| print_command(&el.0, &el.1)),
    }
    Ok(())
}

pub async fn run_collection(bind_address: &str) -> anyhow::Result<()> {
    let handler = tokio::net::TcpListener::bind(bind_address).await?;

    loop {
        if let Ok((stream, address)) = handler.accept().await {
            if let Err(e) = tokio::spawn(request_handle(stream, address)).await {
                tracing::error!("Connection error: {}", e);
            }
        } else {
            tracing::error!("Can't accept new connection...")
        }
    }
}
