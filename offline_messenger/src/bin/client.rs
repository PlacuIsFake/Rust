use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Debug)]
struct CreateUser {
    username: String,
    password: String,
}
#[derive(Deserialize, Debug)]
struct UserResponse {
    id: u64,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:3000";

    let resp = client
        .get(format!("{base_url}/"))
        .send()
        .await?
        .text()
        .await?;
    println!("{resp}");

    println!("\n--- 2. Testing POST /users ---");
    let new_user = CreateUser {
        username: "Pax".to_string(),
        password: "password".to_string(),
    };
    let user_resp = client
        .post(format!("{base_url}/users"))
        .json(&new_user)
        .send()
        .await?
        .json::<UserResponse>()
        .await?;
    println!(
        "{}, {}, {}",
        user_resp.id, user_resp.username, user_resp.password
    );
    println!("Create User: {user_resp:?}");
    Ok(())
}
