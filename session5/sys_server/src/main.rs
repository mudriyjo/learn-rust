mod handler;

const SERVER_ADDRESS: &str = "0.0.0.0:9444";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    handler::run_collection(SERVER_ADDRESS)
        .await
        .expect("Error on starting sys collector server...");
}
