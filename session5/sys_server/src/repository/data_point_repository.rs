use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Datapoints {
    id: i32,
    collector_id: String,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

//TODO REMOVE unwrap
pub async fn get_all_datapoints(Extension(pool): Extension<Pool<Postgres>>) -> Json<Vec<String>> {
    let res: Vec<Datapoints> =
        sqlx::query_as("SELECT id, collector_id, total_memory, used_memory, average_cpu FROM datalog;")
            .fetch_all(&pool)
            .await
            .unwrap();

        let result: Vec<String> = res.into_iter().map(|el| serde_json::to_string(&el).unwrap()).collect();

        Json(result)
}
