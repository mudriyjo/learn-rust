use tracing::{instrument, subscriber};
use tracing_subscriber::fmt::format::FmtSpan;

#[instrument]
async fn hello() {
    println!("Hello");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt().json()
        .compact()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .finish();

    subscriber::set_global_default(subscriber)?;

    tracing::info!("Info msg...");
    tracing::warn!("Debug msg...");
    tracing::error!("Error msg...");

    hello().await;

    Ok(())
}
