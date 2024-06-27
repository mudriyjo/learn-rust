use std::time::Duration;

use tokio::sync::mpsc::Receiver;
use tokio::sync::broadcast::{self};

async fn do_some_work() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn timeout(timeout: u64) {
    tokio::time::sleep(Duration::from_secs(timeout)).await;
}

async fn select_channel(reciever: &mut Receiver<u32>, reciever_broadcast: &mut broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(v) = reciever.recv() => println!("[Regular] channel recieve {}", v),
            Ok(v) = reciever_broadcast.recv() => println!("[Broadcast] channel recieve {}", v),
        };
    }
}

async fn broadcast_example() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(1);
    let (tx_broadcast, mut rx_broadcast) = tokio::sync::broadcast::channel::<u32>(1);
    
    let handle = tokio::spawn(async move {select_channel(&mut rx, &mut rx_broadcast).await});
    
    for i in 0..10 {
        if i % 2 == 0 {
            tx_broadcast.send(i);
        } else {
            tx.send(i).await;
        }
    }
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_some_work() => println!("Work is done..."),
        _ = timeout(1) => println!("timeout..."),
    }

    tokio::select! {
        _ = do_some_work() => println!("Work is done..."),
        _ = timeout(3) => println!("timeout..."),
    }

    println!("Broadcast example....");
    broadcast_example().await
}
