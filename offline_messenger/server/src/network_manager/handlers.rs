use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::network_manager::server::AppState;

#[derive(Deserialize, Clone)]
pub struct SigninReq {
    pub username: String,
    pub password: String,
    pub password_again: String,
}
#[derive(Deserialize, Clone)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct SessionInfo {
    pub username: String,
    pub token: String,
}
#[derive(Deserialize)]
pub struct NewMessage {
    pub from: u64,
    pub to: u64,
    pub message: String,
}
#[derive(Serialize)]
pub struct Message {
    pub id: u64,
    pub from: u64,
    pub to: u64,
    pub message: String,
}
#[derive(Serialize)]
pub struct Response {
    pub succes: bool,
    pub message: String,
}

pub struct Handlers {}
impl Handlers {
    pub async fn signin(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<SigninReq>,
    ) -> impl IntoResponse {
        println!("Sign in attempt");
        match app_state.database.signin(payload.clone()).await {
            Ok(r) => match r.succes {
                true => return (StatusCode::CREATED, Json(r)),
                false => return (StatusCode::CONFLICT, Json(r)),
            },
            Err(err) => {
                println!("Error during sign in: {err}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Response {
                        succes: false,
                        message: "Internal server error".to_string(),
                    }),
                );
            }
        }
    }

    pub async fn login(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<LoginReq>,
    ) -> impl IntoResponse {
        println!("Login attempt as {}", payload.username.clone());
        match app_state.database.login(payload.clone()).await {
            Ok(r) => match r.succes {
                true => {
                    let token = app_state.session_manager.new_session(&payload.username);
                    return (
                        StatusCode::OK,
                        Json(json!({
                            "succes": r.succes,
                            "token": token,
                            "message": r.message,
                        })),
                    );
                }
                false => {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "succes": r.succes,
                            "message": r.message,
                        })),
                    );
                }
            },
            Err(err) => {
                println!("Error during sign in: {err}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "succes": false,
                        "message": "Internal server error".to_string(),
                    })),
                );
            }
        }
    }
}
