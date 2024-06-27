use tokio::sync::Mutex;
use once_cell::sync::Lazy;

static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one() {
    let mut lock = COUNTER.lock().await;
    *lock = (*lock) + 1;
}

#[tokio::main]
async fn main() {
    tokio::join!(add_one(), add_one(), add_one());
    println!("Counter is: {}", COUNTER.lock().await);
}
