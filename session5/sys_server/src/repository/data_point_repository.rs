use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Datapoints {
    pub id: i32,
    pub collector_id: String,
    pub total_memory: i64,
    pub used_memory: i64,
    pub average_cpu: f32,
}

//TODO REMOVE unwrap
pub async fn get_all_datapoints(Extension(pool): Extension<Pool<Postgres>>) -> Json<Vec<String>> {
    let res: Vec<Datapoints> = sqlx::query_as(
        "SELECT id, collector_id, total_memory, used_memory, average_cpu FROM datalog;",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let result: Vec<String> = res
        .into_iter()
        .map(|el| serde_json::to_string(&el).unwrap())
        .collect();

    Json(result)
}

pub async fn save_datapoint(
    Extension(pool): Extension<Pool<Postgres>>,
    datapoint: Datapoints,
) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO datalog (collector_id, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4);")
            .bind(datapoint.collector_id)
            .bind(datapoint.total_memory)
            .bind(datapoint.used_memory)
            .bind(datapoint.average_cpu)
            .execute(&pool)
            .await?;

    Ok(())
}
