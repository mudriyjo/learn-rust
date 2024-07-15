use axum::{extract::Path, response::IntoResponse, routing::get, Extension};
use axum_template::{engine::Engine, RenderHtml};
use chrono::DateTime;
use minijinja::{context, Environment};
use repository::data_point_repository::{
    get_collectors, get_datapoints, get_datapoints_by_collector_id,
};
use serde::{Deserialize, Serialize};
use service::data_point_service::{get_collectors_list, get_datapoints_collector};
use sqlx::{PgPool, Pool, Postgres};
use tokio::net::TcpListener;

mod handler;
mod repository;
mod service;

const SERVER_ADDRESS: &str = "0.0.0.0:9444";

// TODO
// 1. Refactor
// Handler -> to separate controller
// Data points split result and json converstion
// Add service layer
type AppEngine = Engine<Environment<'static>>;

async fn index(engine: AppEngine, Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let data: Vec<(String, String)> = get_collectors(pool)
        .await
        .into_iter()
        .map(|el| {
            let time = DateTime::from_timestamp(el.last_update, 0).unwrap();
            let time_str = format!("{}", time.format("%d-%m-%Y %H:%M:%S"));
            (el.collector_id, time_str)
        })
        .collect();

    RenderHtml("index.jinja", engine, context!(collector => data))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatapointsView {
    pub id: String,
    pub total_memory: i64,
    pub used_memory: i64,
    pub average_cpu: f32,
    pub created_time: String,
}

async fn collector(
    engine: AppEngine,
    Path(collector_id): Path<String>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> impl IntoResponse {
    let datapoints = get_datapoints_by_collector_id(collector_id, pool).await;

    let l_cpu_set = datapoints
        .iter()
        .map(|el| {
            let time = DateTime::from_timestamp(el.created_time, 0).unwrap();
            format!("\"{}\"", time.format("%H:%M:%S"))
        })
        .collect::<Vec<String>>()
        .join(",");
    let d_cpu_set = datapoints
        .iter()
        .map(|el| el.average_cpu.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let l_mem_set = datapoints
        .iter()
        .map(|el| {
            let time = DateTime::from_timestamp(el.created_time, 0).unwrap();
            format!("\"{}\"", time.format("%H:%M:%S"))
        })
        .collect::<Vec<String>>()
        .join(",");
    let d_mem_set = datapoints
        .iter()
        .map(|el| ((el.used_memory as f64 / el.total_memory as f64) * 100.0).to_string())
        .collect::<Vec<String>>()
        .join(",");

    let data: Vec<DatapointsView> = datapoints
        .into_iter()
        .map(|el| {
            let time = DateTime::from_timestamp(el.created_time, 0).unwrap();
            let time_str = format!("{}", time.format("%d-%m-%Y %H:%M:%S"));
            DatapointsView {
                id: el.id.to_string(),
                total_memory: el.total_memory,
                used_memory: el.used_memory,
                average_cpu: el.average_cpu,
                created_time: time_str,
            }
        })
        .collect();

    RenderHtml(
        "collector.jinja",
        engine,
        context!(
            collector => data,
            label_cpu_set => l_cpu_set,
            data_cpu_set => d_cpu_set,
            label_mem_set => l_mem_set,
            data_mem_set => d_mem_set,
        ),
    )
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
        .route("/collector/:collector_id", get(collector))
        .route("/api/datapoint", get(get_datapoints))
        .route(
            "/api/datapoint/:collector_id",
            get(get_datapoints_collector),
        )
        .route("/api/collector", get(get_collectors_list))
        .layer(Extension(extension_pool))
        .with_state(AppEngine::new(env));

    let server = axum::serve(connection, router);

    tokio::task::spawn(async move {
        tracing::info!("Start daemon listner...");
        handler::run_collection(SERVER_ADDRESS, &pool)
            .await
            .expect("Error on starting sys collector server...");
    });

    tracing::info!("Start server...");
    server.await?;

    Ok(())
}
