use axum::{
    Json,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;

use crate::network_manager::server::{AppState, InternalMessage};

#[derive(Deserialize, Clone)]
pub struct SigninReq {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Clone)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct SessionInfo {
    pub username: String,
    pub token: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub from: String,
    pub to: String,
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
                true => (StatusCode::CREATED, Json(r)),
                false => (StatusCode::CONFLICT, Json(r)),
            },
            Err(err) => {
                println!("Error during sign in: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Response {
                        succes: false,
                        message: "Internal server error".to_string(),
                    })
                )
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
                    (
                        StatusCode::OK,
                        Json(json!({
                            "succes": r.succes,
                            "token": token,
                            "message": r.message,
                        })),
                    )
                }
                false => {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "succes": r.succes,
                            "token": "".to_string(),
                            "message": r.message,
                        })),
                    )
                }
            },
            Err(err) => {
                println!("Error during sign in: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "succes": false,
                        "token": "".to_string(),
                        "message": "Internal server error".to_string(),
                    })),
                )
            }
        }
    }

    pub async fn ws_handler(
        ws: WebSocketUpgrade,
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<SessionInfo>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| {
            Handlers::handle_socket(socket, app_state.clone(), payload)
        })
    }
    async fn handle_socket(
        socket: WebSocket,
        app_state: Arc<AppState>,
        session_info: SessionInfo,
    ) {
        let (mut sender, mut receiver) = socket.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<InternalMessage>();

        {
            let mut map = match app_state.map.lock() {
                Ok(m) => m,
                Err(err) => {
                    println!("Error while locking the map in app_state: {err}");
                    return;
                }
            };
            let sessions = map.entry(session_info.username.clone()).or_insert(HashMap::new());
            sessions.insert(session_info.token.clone(), tx);
        }
        println!("User {} is now connected.", session_info.username.clone());

        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    InternalMessage::Chat {
                        sender: s,
                        content: c,
                    } => {
                        if sender
                            .send(Message::Text(format!("From {s}: {c}").into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    InternalMessage::Quit => break,
                }
            }
        });

        while let Some(Ok(_)) = receiver.next().await {
            
        }

        send_task.abort();
        {
            let mut map = match app_state.map.lock() {
                Ok(m) => m,
                Err(err) => {
                    println!("Error while locking the map in app_state: {err}");
                    return;
                }
            };
            if let Some(sessions) = map.get_mut(&session_info.username) {
                sessions.remove(&session_info.token);
                if sessions.is_empty() {
                    map.remove(&session_info.username);
                }
            }
        }
    }
    pub async fn send_message(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<ChatMessage> 
    ) -> impl IntoResponse {
        println!("Sending message from {} to {}", payload.from.clone(), payload.to.clone());
        match app_state.database.send_message(&payload.from, &payload.to, &payload.message).await {
            Ok(r) => match r.succes {
                    true => 
                    {
                        {
                            let map = match app_state.map.lock() {
                                Ok(m) => m,
                                Err(err) => {
                                    println!("Error while locking the map in app_state: {err}");
                                    let resp = Response{
                                        succes: false,
                                        message: "Internal server error".to_string()
                                    };
                                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
                                }
                            };
                            if let Some(s) = map.get(&payload.to) {
                                for tx in s.values() {
                                    match tx.send(InternalMessage::Chat { sender: payload.from.clone(), content: payload.message.clone() }) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            println!("Error while sending the notification to the receiver: {err}");
                                            let resp = Response{
                                                succes: false,
                                                message: "Internal server error".to_string()
                                            };
                                            return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
                                        }
                                    };
                                }
                            }
                        }
                        (StatusCode::CREATED, Json(r))
                    },
                    false => (StatusCode::CONFLICT, Json(r))
                }
            Err(err) => {
                println!("Error while working with the database: {err}");
                let resp = Response {
                    succes: false,
                    message: "Internal server error".to_string()
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
            },
        }
    }
}
