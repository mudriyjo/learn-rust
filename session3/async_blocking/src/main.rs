use std::time::Duration;

async fn task_blocking(task_number: u32, task_time: u64) {
    println!("Task number-{task_number} starting...");
    std::thread::sleep(Duration::from_millis(task_time));
    println!("Task number-{task_number} finished");
}

async fn task_sleep_tokio(task_number: u32, task_time: u64) {
    println!("Task number-{task_number} starting...");
    tokio::time::sleep(Duration::from_millis(task_time)).await;
    println!("Task number-{task_number} finished");
}

async fn task_tokio_blocking(task_number: u32, task_time: u64) {
    println!("Task number-{task_number} starting...");
    // Perform blocking operation separatly
    let _ = tokio::task::spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(task_time));
    }).await;
    println!("Task number-{task_number} finished");
}

#[tokio::main]
async fn main() {
    println!("Blocking....");
    tokio::join!(
        task_blocking(1, 500),
        task_blocking(2, 1000),
        task_blocking(3, 1500)
    );
    println!();
    println!("Tokio sleep....");
    tokio::join!(
        task_sleep_tokio(1, 500),
        task_sleep_tokio(2, 1000),
        task_sleep_tokio(3, 1500)
    );
    println!();
    println!("Tokio on block....");
    tokio::join!(
        task_tokio_blocking(1, 500),
        task_tokio_blocking(2, 1000),
        task_tokio_blocking(3, 1500)
    );
}
