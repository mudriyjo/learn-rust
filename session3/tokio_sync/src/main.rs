use tokio::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

#[tokio::main]
async fn main() {

}
