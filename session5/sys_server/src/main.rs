use axum::{response::IntoResponse, routing::get, Extension};
use axum_template::{engine::Engine, RenderHtml};
use minijinja::{render, Environment};
use repository::data_point_repository::{
    get_collectors, get_datapoints, get_datapoints_by_collector_id,
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod handler;
mod repository;

const SERVER_ADDRESS: &str = "0.0.0.0:9444";

// TODO
// 1. Refactor
// Handler -> to separate controller
// Data points split result and json converstion
// Add service layer
type AppEngine = Engine<Environment<'static>>;

#[derive(Debug, Serialize)]
struct IndexValues {
    title: String,
    heading: String,
    context: String
}

async fn index(
    engine: AppEngine
) -> impl IntoResponse {
    let data = IndexValues {
        title: "Test template".to_string(),
        heading: "Test template".to_string(),
        context: "Test template".to_string(),
    };

    RenderHtml("index.jinja", engine, data)
}

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

    let mut env = minijinja::Environment::new();
        env.set_loader(minijinja::path_loader("./src/static"));
    
    let router = axum::Router::new()
        .route("/", get(index))
        .route("/api/datapoint", get(get_datapoints))
        .route(
            "/api/datapoint/:collector_id",
            get(get_datapoints_by_collector_id),
        )
        .route("/api/collector", get(get_collectors))
        .layer(Extension(extension_pool))
        .with_state(AppEngine::new(env));

    let server = axum::serve(connection, router);

    tokio::task::spawn(async move {
        println!("Start daemon listner...");
        handler::run_collection(SERVER_ADDRESS, &pool)
            .await
            .expect("Error on starting sys collector server...");
    });

    println!("Start server...");
    server.await?;

    Ok(())
}
