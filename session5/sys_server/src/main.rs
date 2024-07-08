mod handler;

const SERVER_ADDRESS: &str = "0.0.0.0:9444";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handler = tokio::net::TcpListener::bind(SERVER_ADDRESS).await?;
    loop {
        if let Ok((stream, address)) = handler.accept().await {
            tokio::spawn(async move {
                handler::request_handle(stream, address).await;
            })
            .await?;
        }
    }
    Ok(())
}
