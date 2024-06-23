fn sync_function() {
    println!("Sync function...");
}

async fn say_hello() {
    println!("Hello world!");
    say_hello_again().await;
    
    let doubled = double(2).await;
    println!("Doubled num: 2, is: {doubled}");

    futures::join!(say_hello_again(), say_hello_again());
    let futures = vec![double(2), double(4), double(6)];

    let res = futures::future::join_all(futures).await;
    println!("Futures result: {res:?}");
    sync_function();
}

async fn say_hello_again() {
    println!("Hello world!");
}

async fn double(n:i32) -> i32 {
    n * 2
}

fn main() {
    futures::executor::block_on(say_hello());
}
