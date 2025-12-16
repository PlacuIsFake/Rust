use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use crate::network_manager::session_manager::SessionManager;


pub struct Server {
    session_manager: Arc<SessionManager>,
}

impl Server {
    pub fn new() -> Self {
        Self { session_manager: Arc::new(SessionManager::new()), }
    }
    
}