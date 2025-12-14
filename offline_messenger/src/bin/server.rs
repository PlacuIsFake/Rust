use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    password: String,
}
#[derive(Deserialize)]
struct NewMessage {
    from: u64,
    to: u64,
    message: String,
}
#[derive(Serialize)]
struct Message {
    id: u64,
    from: u64,
    to: u64,
    message: String,
}
async fn root() -> &'static str {
    "Welcome to offline_messenger!"
}
async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1984,
        username: payload.username,
        password: payload.password,
    };
    (StatusCode::CREATED, Json(user))
}
async fn new_message(Json(payload): Json<NewMessage>) -> impl IntoResponse {
    let mes = Message {
        id: 1,
        from: payload.from,
        to: payload.to,
        message: payload.message,
    };
    (StatusCode::CREATED, Json(mes))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/messages", post(new_message));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
