use serde_derive::{Deserialize, Serialize};

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
    zipcode: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/users";
    let resp = reqwest::get(url)
            .await?
            .json::<Vec<User>>()
            .await?;

    println!("{resp:#?}");
    Ok(())
}
