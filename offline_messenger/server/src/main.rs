use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use rusqlite::Connection;

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
    let mut user = User {
        id: 0,
        username: payload.username,
        password: payload.password,
    };
    let conn = match Connection::open("users.db")
    {
        Ok(c) => c,
        Err(err) => {
            println!("Couldn't connect to database: {err}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(user));
        },
    };
    let create = r"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
    ";
    match conn.execute(create, ()) {
        Ok(_) => (),
        Err(err) => {
            println!("Execution failed: {err}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(user));
        }
    };

    let sql = format!("SELECT COUNT(*), MAX(NVL(id, 0)) FROM users WHERE username = '{}'", user.username);
    let mut stmt = match conn.prepare(&sql) {
        Ok(s) => s,
        Err(err) => {
                        println!("Couldn't prepare statement: {err}");
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(user));
                    },
    };
    let mut user_iter = match stmt.query_map([], |row| {
        let cnt: i32 = match row.get(0) {
            Ok(val) => val,
            Err(err) => {
                println!("Failed to get count: {err}");
                return Ok((-1, 0));
            },
        };
        let new_id: u64 = match row.get(1) {
            Ok(val) => val,
            Err(err) => {
                println!("Failed to get new id: {err}");
                return Ok((-1, 0));
            }
        };
        Ok((cnt, new_id + 1))
    }) {
        Ok(iter) => iter,
        Err(err) => {
            println!("Query failed: {err}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(user));
        },
    };
    let verf = match user_iter.next() {
        Some(item) => match item {
            Ok(val) => val,
            Err(err) => {
                println!("Error reading row: {err}");
                (-1, 0)
            },
        }
        None => {
            println!("Error row or something");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(user));
        },
    };
    match verf.0 {
        -1 => {
                println!("Error during procces\n");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(user))
            },
        0 => {
            user.id = verf.1;
            (StatusCode::CREATED, Json(user))
        },
        _ => {
            println!("Username taken\n");
            (StatusCode::UNAUTHORIZED, Json(user))
        },
    }
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
