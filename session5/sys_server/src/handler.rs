use std::net::SocketAddr;

use chrono::DateTime;
use protocol::{CollectorCommand, Commands};
use sqlx::{Pool, Postgres};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::repository::data_point_repository::{save_datapoint, save_datapoint_list};

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
    let mut buf = Vec::with_capacity(1024);
    tcp_stream.read_to_end(&mut buf).await?;

    match protocol::decode_v1(&buf) {
        Commands::Command((seconds, command)) => {
            print_command(&seconds, &command);
            save_datapoint(pool, command).await?;
        }
        Commands::Commands(commands_list) => {
            let com_list = commands_list.into_iter().map(|v| {
                print_command(&v.0, &v.1);
                v.1
            }).collect();
            save_datapoint_list(pool, com_list).await?;
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
