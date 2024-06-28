use std::path::Path;
use axum::{response::Html, routing, Json};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .route("/", routing::get(say_hello))
        .route("/json", routing::get(handle_json))
        .route("/file", routing::get(handle_file))
        .route("/post", routing::post(handle_post));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    axum::serve(listener, router).await.unwrap();
}

#[derive(Serialize)]
struct Message {
    message: String
}
async fn handle_post() -> Json<Message> {
    Json(Message{ message: "hello from post api...".to_string()})
}

async fn handle_json() -> Json<Message> {
    Json(Message{ message: "hello from Json".to_string()})
}

async fn handle_file() -> Html<String> {
    let path = Path::new("./hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn say_hello() -> &'static str {
    "Hello world!"
}