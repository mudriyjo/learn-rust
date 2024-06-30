use axum::{
    body::Body, extract::{Multipart, Path}, http::header, response::{Html, IntoResponse}, routing::{get, post}, Extension
};
use errors::app_error::AppError;
use sqlx::{PgPool, Pool, Postgres};
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;
pub mod errors;
/*
 GET '/' Display upload form
 GET '/image' - return json list of all images
 POST '/upload' - upload image to server
 GET '/image/:id' - Display single image
 GET '/thum/:id' - Display single thumbnail
 POST '/search' - Seach image by name
 TODO
 0.0 Logging all error into tracing lib - Done
 0. Create env - Done
 1. Create DB - Done
 2. Create initial Schema (migration) Image -> id, name - Done
 3. Share pool acros axum -> Done
 4. Implement test connection to DB test selecting images and return number of images and mount to test route - Done
 5. Create index.html to uplaod file post and name text form with multipart form + mount route + implement axum multipart extractor - Done
 6. Save image in upload route to DB with returning id number. Image byte should save in disk to image dir. And return redirect step 15 - Done
 7. Implement get image by id handler + route + return StreamBody(ReaderStream(file)) - Done
 8. Implement making thumbnail (100x100) function - Done
 9. Implement fn that look at all images and find what image lost thumbnail and generate it (in separate process tokio:spwan)
 10. Before starting server start fn that check missing thumbnails
 11. When we save new image we should create thumbanil - Done
 12. Write function that return Json<Vec<ImageRecord>> + create handler and route for this
 13. Create route and handler to retrieve thumbnail
 14. Create html to display thumbnail image which show thumnails with link to full image
 15. Add file to redirect post after upload image - redirect.html - Done
 16. Add search form into index.html - post /search form + implement fn search_images + add route
 17. Using place holder into form to replace it by find images in DB into search.html
 18. Change layer/extension to state for more safety
 19. Refactoring code
*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init tracing
    tracing_subscriber::fmt().init();
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
        // .route("/", get(test_connection))
        .route("/", get(index))
        .route("/upload", post(file_upload))
        .route("/image/:id", get(get_image))
        .layer(Extension(pool));

    axum::serve(connection, rounting).await?;
    Ok(())
}
/*
UTILITY
*/
async fn return_file(file_name: &str) -> Result<Html<String>, AppError> {
    let full_path = format!("./src/resources/{}",file_name);
    let index_path = std::path::Path::new(&full_path);
    let file_content = tokio::fs::read_to_string(index_path).await?;
    Ok(Html(file_content))
}

async fn make_thumbnail(id: i32) -> Result<(), AppError> {
    let image_name = format!("{}.jpg", id);
    let full_image_path = format!("./src/upload/{}",image_name);
    let full_thumbnail_path = format!("./src/upload/thumbnail_{}.jpg",id);
    let res = image::open(full_image_path)?.resize(100, 100, image::imageops::FilterType::Triangle);
    res.save(full_thumbnail_path)?;
    Ok(())
}
/*
GET IMAGE
*/
async fn get_image(Path(id): Path<i64>) -> Result<impl IntoResponse, AppError>{
    let file_name = format!("{}.jpg", id);
    let file_header = format!("filename=images/{}", file_name);
    let header = [
        (header::CONTENT_TYPE, "image/jpeg".to_string()),
        (header::CONTENT_DISPOSITION, file_header),
    ];
    
    let full_path = format!("./src/upload/{}",file_name);
    let image_file = tokio::fs::File::open(full_path).await?;
    let stream = ReaderStream::new(image_file);
    Ok((header, axum::body::Body::from_stream(stream)))
}
/*
IMAGE UPLOAD
*/
async fn index() -> Result<Html<String>, AppError> {
    return_file("index.html").await
}

async fn file_upload(
    Extension(pool): Extension<Pool<Postgres>>,
    mut multipart: Multipart,
) -> Result<Html<String>, AppError> {
    let mut image_name: Option<String> = None;
    let mut image_bytes: Option<Vec<u8>> = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        match name.as_str() {
            "name" => image_name = Some(String::from_utf8(data.to_vec())?),
            "image" => image_bytes = Some(data.to_vec()),
            field_name => {
                return Err(AppError(anyhow::Error::msg(format!(
                    "file uplaod doesn't support next field: {}",
                    field_name
                ))))
            }
        }
    }

    store_image(image_name, image_bytes, pool).await?;
    return_file("redirect.html").await
}

async fn store_image(
    image_name: Option<String>,
    image_bytes: Option<Vec<u8>>,
    pool: Pool<Postgres>,
) -> Result<(), AppError> {
    if let (Some(name), Some(img_bytes)) = (image_name, image_bytes) {
        let upload_path = std::path::Path::new("./src/upload/");
        let image_id = sqlx::query!("INSERT INTO image (name) VALUES ($1) RETURNING id", name)
            .fetch_one(&pool)
            .await?
            .id;

        tokio::fs::write(&upload_path.join(format!("{}.jpg", image_id)), img_bytes).await?;
        make_thumbnail(image_id).await?;
        Ok(())
    } else {
        Err(AppError(anyhow::Error::msg(
            "Form doesn't contain key fields: title and image",
        )))
    }
}

/*
COUNT IMAGE
*/
async fn test_connection(Extension(pool): Extension<Pool<Postgres>>) -> Result<String, AppError> {
    let record = sqlx::query!("SELECT COUNT(id) FROM image")
        .fetch_one(&pool)
        .await?;

    match record.count {
        Some(cnt) => Ok(format!("Count images: {}", cnt)),
        None => {
            tracing::error!("Can't calculate count in test_connection fn");

            Err(AppError(anyhow::Error::msg(
                "Can't calculate images count...".to_string(),
            )))
        }
    }
}
