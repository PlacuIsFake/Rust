use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}, error::Error, collections::HashMap};
use uuid::Uuid;

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
        let token = &Uuid::new_v4().to_string().replace("-", "")[..16];
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(token.to_string(), user.to_string());
        token.to_string()
    }
    pub fn validate_session(&self, token: &str) -> Option<String> {
        let sessions = self.sessions.lock().unwrap();

        match sessions.get(token) {
            Some(user) => {
                println!("Valid token for user {}", user.clone().to_string());
                Some(user.to_string())
            },
            None => 
            {
                println!("Invalid token");
                None
            },
        }
    }
    pub fn close_session(&self, token: &str) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        match sessions.remove(token) {
            Some(user) => {
                println!("{} logged out", user.clone().to_string());
                true
            }
            None => false
        }
    }
}