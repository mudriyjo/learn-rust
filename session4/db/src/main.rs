use futures::{StreamExt, TryStreamExt};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};

// 1. .dotenv - Done
// 2. install sqlx-cli - Done
// 3. Create db - Done
// 4. Create migration - Done
// 5. Create pool, and make select from DB - Done
// 6. Add derive FromRow to struct - Done
// 7. Add streaming - Done
// 8. Add migration into code levev - Done
// 9. Add CRUD for DB - Done
#[derive(Debug, FromRow)]
struct Message {
    pub id: i32,
    pub message: String,
}

async fn get_message_by_id(id: i32, pool: &PgPool) -> anyhow::Result<Message> {
    let res = sqlx::query_as!(
        Message,
        "SELECT id, message FROM messages WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(res)
}

async fn get_all_message(pool: &PgPool) -> anyhow::Result<Vec<Message>> {
    let res = sqlx::query_as!(Message, "SELECT id, message FROM messages")
        .fetch_all(pool)
        .await?;

    Ok(res)
}

async fn update_message_by_id(id: i32, new_message: &str, pool: &PgPool) -> anyhow::Result<bool> {
    let res = sqlx::query!(
        "UPDATE messages SET message = $1 WHERE id = $2",
        new_message,
        id
    )
    .execute(pool)
    .await?;

    Ok(res.rows_affected() > 0)
}

async fn delete_message_by_id(id: i32, pool: &PgPool) -> anyhow::Result<bool> {
    let res = sqlx::query!("DELETE FROM messages WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(res.rows_affected() > 0)
}

async fn insert_message(message: Message, pool: &PgPool) -> anyhow::Result<bool> {
    let res = sqlx::query!(
        "INSERT INTO messages (id, message) VALUES ($1, $2)",
        message.id,
        message.message
    )
    .execute(pool)
    .await?;

    Ok(res.rows_affected() > 0)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let message = get_message_by_id(1, &pool).await?;
    println!("Message with id: 1, message: {message:?}");

    let is_updated = update_message_by_id(1, "Test test test...", &pool).await?;
    println!("Is message update with id: 1, {is_updated}");

    let is_deleted = delete_message_by_id(4, &pool).await?;
    println!("Is message deleted with id 4, {is_deleted}");

    // let is_inserted = insert_message(
    //     Message {
    //         id: 100,
    //         message: "Big, big message...".to_string(),
    //     },
    //     &pool,
    // )
    // .await?;
    // println!("Is message inserted: {is_inserted:?}");

    let messages = get_all_message(&pool).await?;
    println!("All messages: {messages:?}");

    let mut stream = sqlx::query_as!(Message, "SELECT id, message FROM messages")
        .fetch(&pool);

    while let Some(msg) = stream.try_next().await? {
        println!("{msg:?}")
    }

    Ok(())
}
