use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}, error::Error, collections::HashMap};

#[derive(Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct User{
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

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, String>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn new_session(&self, user: &str) -> String {
        
    }
}