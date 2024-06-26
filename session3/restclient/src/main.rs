use std::time::Duration;

use serde_derive::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpSocket, TcpStream}};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    zipcode: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/users";
    let resp = reqwest::get(url)
            .await?
            .json::<Vec<User>>()
            .await?;

    println!("{resp:#?}");

    tcp_server().await;
    Ok(())
}

async fn tcp_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let msg = b"Hello";
    let mut buf = vec![0u8; 1024];
    println!("[Client] Writing msg: '{}' to buffer", String::from_utf8_lossy(msg));
    let _ = stream.write_all(msg).await;
    let readed_bytes = stream.read(&mut buf).await.unwrap();
    let read_msg = String::from_utf8_lossy(&buf[..readed_bytes]);
    println!("[Client] Readed msg: '{}' from buffer", read_msg);
}

async fn client_wrapper() {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tcp_client().await;
    }
}

async fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tokio::spawn(client_wrapper());
    loop {
        let (mut tcp_stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let readed_bytes = tcp_stream.read(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..readed_bytes]);
            println!("[Server] Readed: {:?}", msg);

            if tcp_stream.write(&buf).await.is_ok() {
                println!("[Server] Msg send back...");
            } else {
                println!("[Server] Problem to write msg to socket...")
            }
        });
    }
}
