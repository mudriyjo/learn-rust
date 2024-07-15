use axum::{extract::Path, Extension, Json};
use sqlx::{Pool, Postgres};

use crate::repository::data_point_repository::{get_collectors, get_datapoints_by_collector_id};

pub async fn get_collectors_list(Extension(pool): Extension<Pool<Postgres>>) -> Json<Vec<String>> {
    let res = get_collectors(pool).await;

    let result: Vec<String> = res
        .into_iter()
        .map(|el| serde_json::to_string(&el).unwrap())
        .collect();

    Json(result)
}

pub async fn get_datapoints_collector(
    Path(collector_id): Path<String>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Json<Vec<String>> {
    let res = get_datapoints_by_collector_id(collector_id, pool).await;
    let result: Vec<String> = res
        .into_iter()
        .map(|el| serde_json::to_string(&el).unwrap())
        .collect();

    Json(result)
}
