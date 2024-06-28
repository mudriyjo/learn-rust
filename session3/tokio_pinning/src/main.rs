use std::{future::{self, Future}, pin::Pin};
use async_recursion::async_recursion;

#[async_recursion]
async fn fib(n:u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1).await + fib(n-2).await
    }
}

async fn one() {
    println!("One");
}

async fn two() {
    println!("Two");
}

async fn choice(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Incorrect choice...")
    }
}

#[tokio::main]
async fn main() {
    println!("fib 10: {}", fib(10).await);
    choice(1).await.await;

    let future = async {
        println!("Future");
    };

    tokio::pin!(future);
    (&mut future).await;
}
