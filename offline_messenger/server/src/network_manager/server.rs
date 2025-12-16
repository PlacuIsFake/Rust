use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use create::auth::


pub struct Server {
    session_manager: Arc<
}