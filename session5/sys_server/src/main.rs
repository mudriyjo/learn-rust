use axum::{routing::get, Extension};
use repository::data_point_repository::{
    get_datapoints, get_datapoints_by_collector_id,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

mod handler;
mod repository;

const SERVER_ADDRESS: &str = "0.0.0.0:9444";

// TODO
// 1. Save all data to DB
// Prepare next queries
// 2. Get all dataPoints from DB
// 3. Get all dataPoints from DB by collector
// 4. Get all collectors ids and last seen timestamp
// Prepare web server
// 5. Prepare API to retrieve the JSON data from P.2 to P.4
// 6. Prepare page to draw index and show all collectors
// 7. Prepare page to draw collector page -> Should showe collector log with all data
//      and 2 Graphics CPU and Memory utilization
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();
    let server_port_address = std::env::var("SERVER")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    let extension_pool = pool.clone();
    let connection = TcpListener::bind(server_port_address).await?;

    println!("Start server...");
    let router = axum::Router::new()
        .route("/api/datapoint", get(get_datapoints))
        .route(
            "/api/datapoint/:collector_id",
            get(get_datapoints_by_collector_id),
        )
        // .route("/api/collector", get(get_collectors))
        .layer(Extension(extension_pool));

    let server = axum::serve(connection, router);

    println!("Start daemon listner...");
    handler::run_collection(SERVER_ADDRESS, &pool)
        .await
        .expect("Error on starting sys collector server...");

    server.await?;

    Ok(())
}
