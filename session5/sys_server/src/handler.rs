use std::net::SocketAddr;

use chrono::DateTime;
use protocol::{CollectorCommand, Commands};
use sqlx::{Pool, Postgres};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::repository::data_point_repository::save_datapoint;

fn print_command(seconds: &u32, command: &CollectorCommand) {
    let time = DateTime::from_timestamp(*seconds as i64, 0).unwrap();
    tracing::info!(
        "Timestamp: {}, Command: {:?}",
        time.format("%d/%m/%Y %H:%M:%S"),
        command
    );
}

async fn request_handle(
    mut tcp_stream: TcpStream,
    _address: SocketAddr,
    pool: Pool<Postgres>,
) -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(2048);

    // TODO add parse many messages
    tcp_stream.read_buf(&mut buf).await?;

    // TODO add proper error handling
    match protocol::decode_v1(&buf) {
        Commands::Command((_seconds, command)) => {
            save_datapoint(pool, command).await?;
            // print_command(&seconds, &command)
        }
        Commands::Commands(commands_list) => {
            for command in commands_list {
                let pool_clone = pool.clone();
                save_datapoint(pool_clone, command.1).await?;
            }
        }
    }
    Ok(())
}

pub async fn run_collection(bind_address: &str, pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let handler = tokio::net::TcpListener::bind(bind_address).await?;

    loop {
        if let Ok((stream, address)) = handler.accept().await {
            let pool_clone = pool.clone();
            if let Err(e) = tokio::spawn(request_handle(stream, address, pool_clone)).await {
                tracing::error!("Connection error: {}", e);
            }
        } else {
            tracing::error!("Can't accept new connection...")
        }
    }
}
