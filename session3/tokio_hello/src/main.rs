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
async fn hello() -> u32 {
    println!("Hello Tokio!");
    3
}
async fn hello2() -> u32 {
    println!("Hello Tokio2!");
    4
}

async fn tick() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now();
    }
}
// EASY WAY TO IMPL ASYNC MAIN
#[tokio::main]
async fn main() {
    // hello_tokio().await;
    let (x,y) = tokio::join!(hello(), hello2());
    println!("{x}, {y}");

    let res = tokio::spawn(hello()).await.unwrap();
    println!("{res}");

    tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(tick()),
    );
}

// fn main() {
//     tokio_builder_example();
// }

