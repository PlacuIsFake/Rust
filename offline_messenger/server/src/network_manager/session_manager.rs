use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

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
                println!("Valid token for user {}", user.clone());
                Some(user.to_string())
            }
            None => {
                println!("Invalid token");
                None
            }
        }
    }
    pub fn close_session(&self, token: &str) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        match sessions.remove(token) {
            Some(user) => {
                println!("{} logged out", user.clone());
                true
            }
            None => false,
        }
    }
}
