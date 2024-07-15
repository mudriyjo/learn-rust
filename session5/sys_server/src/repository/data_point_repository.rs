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
    pub created_time: i64,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct LastSeenCollector {
    pub collector_id: String,
    pub last_update: i64,
}

//TODO REMOVE unwrap
pub async fn get_datapoints(Extension(pool): Extension<Pool<Postgres>>) -> Json<Vec<String>> {
    let res: Vec<Datapoints> = sqlx::query_as(
        "SELECT id, collector_id, total_memory, used_memory, average_cpu, created_time FROM datalog;",
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
pub async fn get_datapoints_by_collector_id(
    collector_id: String,
    pool: Pool<Postgres>,
) -> Vec<Datapoints> {
    let res: Vec<Datapoints> = sqlx::query_as(
        "SELECT id, collector_id, total_memory, used_memory, average_cpu, created_time FROM datalog WHERE collector_id = $1",
    )
    .bind(collector_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    res
}

pub async fn get_collectors(pool: Pool<Postgres>) -> Vec<LastSeenCollector> {
    let res: Vec<LastSeenCollector> = sqlx::query_as(
        "select distinct on (collector_id) collector_id, created_time as last_update from (select distinct collector_id, created_time from datalog order by collector_id, created_time desc);",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    res
}

pub async fn save_datapoint(
    pool: Pool<Postgres>,
    com: CollectorCommand,
    timestamp: u32,
) -> anyhow::Result<()> {
    match com {
        CollectorCommand::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        } => {
            sqlx::query("INSERT INTO datalog (collector_id, total_memory, used_memory, average_cpu, created_time) VALUES ($1, $2, $3, $4, $5);")
            .bind(collector_id.to_string())
            .bind(total_memory as i64)
            .bind(used_memory as i64)
            .bind(average_cpu_usage)
            .bind(timestamp as i32)
            .execute(&pool)
            .await?;

            Ok(())
        }
    }
}

pub async fn save_datapoint_list(
    pool: Pool<Postgres>,
    com: Vec<(u32, CollectorCommand)>,
) -> anyhow::Result<()> {
    sqlx::query_builder::QueryBuilder::new(
        "INSERT INTO datalog (collector_id, total_memory, used_memory, average_cpu, created_time) ",
    )
    .push_values(com.iter(), |mut b, command| match command {
        (
            timestamp,
            CollectorCommand::SubmitData {
                collector_id,
                total_memory,
                used_memory,
                average_cpu_usage,
            },
        ) => {
            b.push_bind(collector_id.to_string());
            b.push_bind(*total_memory as i64);
            b.push_bind(*used_memory as i64);
            b.push_bind(average_cpu_usage);
            b.push_bind(*timestamp as i32);
        }
    })
    .build()
    .execute(&pool)
    .await?;

    Ok(())
}
