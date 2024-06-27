use tokio::sync::Mutex;

static COUNTER: Mutex<u32> = once_cell::Lazy::new(|| Mutex::new(0));

#[tokio::main]
async fn main() {

}
