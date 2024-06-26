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

// async fn tcp_client() {
//     let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    
// }

async fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut tcp_stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let readed_bytes = tcp_stream.read(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..readed_bytes]);
            println!("readed: {:?}", msg);

            if tcp_stream.write(&buf).await.is_ok() {
                println!("msg send back...");
            } else {
                println!("problem to write msg to socket...")
            }
        });
    }
}
