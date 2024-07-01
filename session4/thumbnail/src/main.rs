use std::collections::HashMap;

use axum::{
    extract::{Multipart, Path},
    http::header,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Form, Json,
};
use errors::app_error::AppError;
use serde::{Deserialize, Serialize};
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

 TODO LIST
 0. Done - Logging all error into tracing lib
 0. Done - Create env
 1. Done - Create DB
 2. Done - Create initial Schema (migration) Image -> id, name
 3. Done - Share pool acros axum
 4. Done - Implement test connection to DB test selecting images and return number of images and mount to test route
 5. Done - Create index.html to uplaod file post and name text form with multipart form + mount route + implement axum multipart extractor
 6. Done - Save image in upload route to DB with returning id number. Image byte should save in disk to image dir. And return redirect step 15
 7. Done - Implement get image by id handler + route + return StreamBody(ReaderStream(file))
 8. Done - Implement making thumbnail (100x100) function
 9. Done - Implement fn that look at all images and find what image lost thumbnail and generate it (in separate process tokio:spwan)
 10. Done - Before starting server start fn that check missing thumbnails
 11. Done - When we save new image we should create thumbanil
 12. Done - Write function that return Json<Vec<ImageRecord>> + create handler and route for this
 13. Done - Create route and handler to retrieve thumbnail
 14. Done - Create html to display thumbnail image which show thumbnails with link to full image
 15. Done - Add file to redirect post after upload image - redirect.html
 16. Done - Add search form into index.html - post /search form + implement fn search_images + add route
 17. Done - Using place holder into form to replace it by find images in DB into search.html
 18. Change layer/extension to state for more safety
 19. Refactoring code
 20. Add working with HTML template
*/
#[tokio::main]
async fn main() -> Result<(), AppError> {
    // init tracing
    tracing_subscriber::fmt().init();
    // init better stack trasing
    color_eyre::install().unwrap();
    // Init env
    dotenv::dotenv()?;

    // Recreate all thumbnail
    recreate_all_thumbnails().await?;

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
        .route("/image", get(get_all_images))
        .route("/image/:id", get(get_image))
        .route("/thum/:id", get(get_thumbnail))
        .route("/search", post(search))
        .layer(Extension(pool));

    axum::serve(connection, rounting).await?;
    Ok(())
}
/*
UTILITY
*/
// TODO REFACTORE THIS
async fn return_file(file_name: &str) -> Result<Html<String>, AppError> {
    let full_path = format!("./src/resources/{}", file_name);
    let index_path = std::path::Path::new(&full_path);
    let file_content: String = tokio::fs::read_to_string(index_path).await?;
    Ok(Html(file_content))
}

async fn make_thumbnail(id: i32) -> Result<(), AppError> {
    let image_name = format!("{}.jpg", id);
    let full_image_path = format!("./src/upload/{}", image_name);
    let full_thumbnail_path = format!("./src/upload/thumbnail_{}.jpg", id);
    let res = image::open(full_image_path)?.resize(100, 100, image::imageops::FilterType::Triangle);
    res.save(full_thumbnail_path)?;
    Ok(())
}

async fn recreate_all_thumbnails() -> Result<(), AppError> {
    let mut dir = tokio::fs::read_dir("./src/upload/").await?;
    let mut images = vec![];
    while let Some(file) = dir.next_entry().await? {
        if let Ok(img_name) = file.file_name().into_string() {
            images.push(img_name);
        }
    }
    let accamulator: HashMap<i32, Vec<String>> = HashMap::new();
    let image_without_thumbnails: Result<HashMap<i32, Vec<String>>, AppError> =
        images.into_iter().try_fold(accamulator, |mut acc, el| {
            let id: i32 = el.replace(".jpg", "").replace("thumbnail_", "").parse()?;
            if let Some(xs) = acc.get_mut(&id) {
                xs.push(el);
            } else {
                acc.insert(id, vec![el]);
            }
            Ok(acc)
        });

    let result_image_without_thumbnails: HashMap<i32, Vec<String>> = image_without_thumbnails?
        .into_iter()
        .filter(|entry| entry.1.len() < 2)
        .collect();

    result_image_without_thumbnails
        .into_iter()
        .for_each(|entry| {
            tokio::spawn(async move {
                let _ = make_thumbnail(entry.0).await;
            });
        });
    Ok(())
}
async fn prepare_thumbnails_html_from_imgs(
    content: String,
    images: Vec<ImageRecord>,
) -> Result<String, AppError> {
    let html_template = "
        <div class=\"col\">
        <a href=\"{image_src}\"><img src=\"{thumb_src}\" class=\"img-thumbnail\" alt=\"{thumb_name}\"></a>
        <p>{thumb_name}</p>
        </div>";

    let change: Result<String, AppError> =
        images.into_iter().try_fold(String::new(), |mut acc, el| {
            let src = format!("/thum/{}", el.id);
            let img = format!("/image/{}", el.id);
            acc.push_str(
                &html_template
                    .replace("{thumb_src}", &src)
                    .replace("{thumb_name}", &el.name)
                    .replace("{image_src}", &img),
            );
            Ok(acc)
        });
    let res = format!(
        "<div class=\"component\"><div class=\"row\">{}</div></div>",
        change?
    );
    Ok(content.replace("{THUMBNAILS}", &res))
}

async fn prepare_thumbnails_html(
    content: String,
    pool: Pool<Postgres>,
) -> Result<String, AppError> {
    let images = sqlx::query_as!(ImageRecord, "SELECT id, name FROM image")
        .fetch_all(&pool)
        .await?;

    prepare_thumbnails_html_from_imgs(content, images).await
}
/*
SEARCH IMAGE BY NAME
*/
#[derive(Debug, Deserialize, Serialize)]
struct SearchImage {
    pub search_name: String,
}

async fn search(
    Extension(pool): Extension<Pool<Postgres>>,
    Form(search_image): Form<SearchImage>,
) -> Result<Html<String>, AppError> {
    let full_path = "./src/resources/search_result.html";
    let index_path = std::path::Path::new(full_path);
    let file_content: String = tokio::fs::read_to_string(index_path).await?;
    let search_criteria = format!("%{}%", search_image.search_name);
    let images = sqlx::query_as!(
        ImageRecord,
        "SELECT id, name FROM image WHERE name LIKE $1",
        search_criteria
    )
    .fetch_all(&pool)
    .await?;

    let content = prepare_thumbnails_html_from_imgs(file_content, images).await?;
    Ok(Html(content))
}
/*
GET ALL IMAGES
*/
#[derive(Debug, Deserialize, Serialize)]
struct ImageRecord {
    id: i32,
    name: String,
}

async fn get_all_images(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<ImageRecord>>, AppError> {
    let images = sqlx::query_as!(ImageRecord, "SELECT id, name FROM image")
        .fetch_all(&pool)
        .await?;

    Ok(Json(images))
}
/*
GET IMAGE
*/
async fn get_image(Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let file_name = format!("{}.jpg", id);
    let file_header = format!("filename=images/{}", file_name);
    let header = [
        (header::CONTENT_TYPE, "image/jpeg".to_string()),
        (header::CONTENT_DISPOSITION, file_header),
    ];

    let full_path = format!("./src/upload/{}", file_name);
    let image_file = tokio::fs::File::open(full_path).await?;
    let stream = ReaderStream::new(image_file);
    Ok((header, axum::body::Body::from_stream(stream)))
}
/*
GET THUMBNAIL
*/
async fn get_thumbnail(Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let file_name = format!("thumbnail_{}.jpg", id);
    let file_header = format!("filename=images/{}", file_name);
    let header = [
        (header::CONTENT_TYPE, "image/jpeg".to_string()),
        (header::CONTENT_DISPOSITION, file_header),
    ];

    let full_path = format!("./src/upload/{}", file_name);
    let image_file = tokio::fs::File::open(full_path).await?;
    let stream = ReaderStream::new(image_file);
    Ok((header, axum::body::Body::from_stream(stream)))
}
/*
IMAGE UPLOAD
*/
// TODO REFACTO THIS
async fn index(Extension(pool): Extension<Pool<Postgres>>) -> Result<Html<String>, AppError> {
    let full_path = "./src/resources/index.html";
    let index_path = std::path::Path::new(full_path);
    let file_content: String = tokio::fs::read_to_string(index_path).await?;
    let content = prepare_thumbnails_html(file_content, pool).await?;
    Ok(Html(content))
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
                return AppError::new(format!(
                    "file uplaod doesn't support next field: {}",
                    field_name
                ))
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
        AppError::new("Form doesn't contain key fields: title and image".to_string())
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
        Some(cnt) => Ok(format!("Total images: {}", cnt)),
        None => {
            tracing::error!("Can't calculate count in test_connection fn");
            AppError::new("Can't calculate images count...".to_string())
        }
    }
}
