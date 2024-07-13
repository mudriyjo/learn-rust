use axum::{Extension, Json};
use protocol::CollectorCommand;
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
pub async fn get_datapoints(Extension(pool): Extension<Pool<Postgres>>) -> Json<Vec<String>> {
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
pub async fn get_datapoints_by_collector_id() {

}

pub async fn get_collectors() {

}

pub async fn save_datapoint(pool: Pool<Postgres>, com: CollectorCommand) -> anyhow::Result<()> {
    match com {
        CollectorCommand::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        } => {
            sqlx::query("INSERT INTO datalog (collector_id, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4);")
            .bind(collector_id.to_string())
            .bind(total_memory as i64)
            .bind(used_memory as i64)
            .bind(average_cpu_usage)
            .execute(&pool)
            .await?;

            Ok(())
        }
    }
}

pub async fn save_datapoint_list(
    pool: Pool<Postgres>,
    com: Vec<CollectorCommand>,
) -> anyhow::Result<()> {
    sqlx::query_builder::QueryBuilder::new("INSERT INTO datalog (collector_id, total_memory, used_memory, average_cpu) ")
        .push_values(com.iter(), |mut b, command| {
            match command {
                CollectorCommand::SubmitData{
                    collector_id,
                    total_memory,
                    used_memory,
                    average_cpu_usage
                } => {
                    b.push_bind(collector_id.to_string());
                    b.push_bind(*total_memory as i64);
                    b.push_bind(*used_memory as i64);
                    b.push_bind(average_cpu_usage);
                }
            }
        })
        .build()
        .execute(&pool)
        .await?;

    Ok(())
}
