use axum::{Extension, Json};
use sqlx::{Pool, Postgres};

use crate::repository::data_point_repository::get_collectors;

pub async fn get_collectors_list(Extension(pool): Extension<Pool<Postgres>>) -> Json<Vec<String>> {
    let res = get_collectors(pool).await;

    let result: Vec<String> = res
    .into_iter()
    .map(|el| serde_json::to_string(&el).unwrap())
    .collect();

    Json(result)
}