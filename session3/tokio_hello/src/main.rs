async fn hello_tokio() {
    println!("Hello Tokio!")
}

fn tokio_builder_example() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .build()
        .unwrap();
    rt.block_on(hello_tokio());
}

// EASY WAY TO IMPL ASYNC MAIN
// #[tokio::main]
// async fn main() {
    // hello_tokio().await;   
// }

fn main() {
    tokio_builder_example();
}

