use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension};
use sqlx::{PgPool, Pool, Postgres};
use tokio::net::TcpListener;

/*
 GET '/' Display upload form
 GET '/image' - return json list of all images
 POST '/upload' - upload image to server
 GET '/image/{id}' - Display single image
 GET '/thum/{id}' - Display single thumbnail
 POST '/search' - Seach image by name
 TODO
 0.0 Logging all error into tracing lib - Done
 0. Create env - Done
 1. Create DB - Done
 2. Create initial Schema (migration) Image -> id, name - Done
 3. Share pool acros axum -> Done
 4. Implement test connection to DB test selecting images and return number of images and mount to test route - Done
 5. Create index.html to uplaod file post and name text form with multipart form + mount route + implement axum multipart extractor
 6. Save image in upload route to DB with returning id number. Image byte should save in disk to image dir. And return redirect step 15
 7. Implement get image by id handler + route + return StreamBody(ReaderStream(file))
 8. Implement making thumbnail (100x100) function
 9. Implement fn that look at all images and find what image lost thumbnail and generate it (in separate process tokio:spwan)
 10. Before starting server start fn that check missing thumbnails
 11. When we save new image we should create thumbanil
 12. Write function that return Json<Vec<ImageRecord>> + create handler and route for this
 13. Create route and handler to retrieve thumbnail
 14. Create html to display thumbnail image which show thumnails with link to full image
 15. Add file to redirect post after upload image - redirect.html
 16. Add search form into index.html - post /search form + implement fn search_images + add route
 17. Using place holder into form to replace it by find images in DB into search.html
 18. Change layer/extension to state for more safety
 19. Refactoring code
*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init better stack trasing
    color_eyre::install().unwrap();

    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let server_port_address = std::env::var("SERVER")?;
    let pool = PgPool::connect(&db_url).await?;

    // Perform migration
    sqlx::migrate!("./migrations").run(&pool).await?;
    let connection = TcpListener::bind(server_port_address).await?;
    let rounting = axum::Router::new()
        .route("/", get(test_connection))
        .layer(Extension(pool));

    axum::serve(connection, rounting).await?;
    Ok(())
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong, {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

async fn test_connection(Extension(pool): Extension<Pool<Postgres>>) -> Result<String, AppError> {
    let record = sqlx::query!("SELECT COUNT(id) FROM image")
        .fetch_one(&pool)
        .await?;

    match record.count {
        Some(cnt) => Ok(format!("Count images: {}", cnt)),
        None => Err(AppError(anyhow::Error::msg(
            "Can't calculate images count...".to_string(),
        ))),
    }
}
