use sqlx::{pool, PgPool};

mod handler;

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
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    handler::run_collection(SERVER_ADDRESS)
        .await
        .expect("Error on starting sys collector server...");
    
    Ok(())
}
