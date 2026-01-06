use axum::{
    Json,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;

use crate::network_manager::server::AppState;

pub enum InternalMessage {
    Notification {
        sender: String,
        content: String,
        resp_msg: Option<String>,
        resp_user: Option<String>,
    },
    Chat {
        messages: Vec<(String, String, Option<String>, Option<String>)>,
    },
    Response {
        id: String,
        succes: bool,
        message: String,
    },
    Users {
        users_list: Vec<String>,
    },
}

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
#[serde(tag = "type")]
enum WsMessage {
    SendMessage {
        id: String,
        from: String,
        to: String,
        message: String,
        resp_msg: Option<String>,
        resp_user: Option<String>,
    },
    GetMessage {
        from: String,
        idx: i64,
    },
    GetUserList {
        user: String,
    },
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
enum WsMessageBack {
    Message {
        from: String,
        message: String,
        resp_msg: Option<String>,
        resp_user: Option<String>,
    },
    Response {
        id: String,
        succes: bool,
        message: String,
    },
    Chat {
        messages: Vec<(String, String, Option<String>, Option<String>)>,
    },
    UserList {
        list: Vec<String>,
    },
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
                    }),
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
                false => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "succes": r.succes,
                        "token": "".to_string(),
                        "message": r.message,
                    })),
                ),
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
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Handlers::handle_socket(socket, app_state.clone()))
    }
    async fn handle_socket(socket: WebSocket, app_state: Arc<AppState>) {
        let (mut sender, mut receiver) = socket.split();
        let session_info;
        if let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(raw_json) = msg {
                let message: Result<SessionInfo, _> = serde_json::from_str(&raw_json);
                session_info = match message {
                    Ok(m) => m,
                    Err(err) => {
                        println!("Error at the start message: {err}");
                        return;
                    }
                };
            } else {
                println!("Error at the start message: idk");
                return;
            }
        } else {
            println!("Error at the start message: idk");
            return;
        }
        let (tx, mut rx) = mpsc::unbounded_channel::<InternalMessage>();

        let tx_clone = tx.clone();
        {
            let mut map = match app_state.map.lock() {
                Ok(m) => m,
                Err(err) => {
                    println!("Error while locking the map in app_state: {err}");
                    return;
                }
            };
            let sessions = map
                .entry(session_info.username.clone())
                .or_insert(HashMap::new());
            sessions.insert(session_info.token.clone(), tx);
        }
        println!("User {} is now connected.", session_info.username.clone());

        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    InternalMessage::Notification {
                        sender: s,
                        content: c,
                        resp_msg: r_m,
                        resp_user: r_u,
                    } => {
                        let r = WsMessageBack::Message {
                            from: s,
                            message: c,
                            resp_msg: r_m,
                            resp_user: r_u,
                        };
                        if let Ok(message) = serde_json::to_string(&r) {
                            match sender.send(Message::Text(message.into())).await {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("Error while sending message to client: {err}");
                                    break;
                                }
                            }
                        }
                    }
                    InternalMessage::Response {
                        id: idx,
                        succes: s,
                        message: m,
                    } => {
                        let r = WsMessageBack::Response {
                            id: idx,
                            succes: s,
                            message: m,
                        };
                        if let Ok(message) = serde_json::to_string(&r) {
                            match sender.send(Message::Text(message.into())).await {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("Error while sending message to client: {err}");
                                    break;
                                }
                            }
                        }
                    }
                    InternalMessage::Chat {
                        messages: chat_messages,
                    } => {
                        let r = WsMessageBack::Chat {
                            messages: chat_messages,
                        };
                        if let Ok(chat) = serde_json::to_string(&r) {
                            match sender.send(Message::Text(chat.into())).await {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("Error while sending message to client: {err}");
                                    break;
                                }
                            }
                        }
                    }
                    InternalMessage::Users { users_list } => {
                        let r = WsMessageBack::UserList { list: users_list };
                        if let Ok(epstein) = serde_json::to_string(&r) {
                            match sender.send(Message::Text(epstein.into())).await {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("Error while sending message to client: {err}");
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        });

        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(raw_json) = msg {
                let message: Result<WsMessage, _> = serde_json::from_str(&raw_json);
                match message {
                    Ok(WsMessage::SendMessage {
                        id,
                        from,
                        to,
                        message,
                        resp_msg,
                        resp_user,
                    }) => {
                        println!("Sending message from {} to {}", from.clone(), to.clone());
                        if let (Some(r_m), Some(r_u)) = (resp_msg, resp_user) {
                            match app_state
                                .database
                                .send_message_with_resp(&from, &to, &message, &r_m, &r_u)
                                .await
                            {
                                Ok(r) => match r.succes {
                                    true => {
                                        {
                                            let map = match app_state.map.lock() {
                                                Ok(m) => m,
                                                Err(err) => {
                                                    println!(
                                                        "Error while locking the map in app_state: {err}"
                                                    );
                                                    match tx_clone.send(InternalMessage::Response {
                                                        id,
                                                        succes: false,
                                                        message: "Internal server error"
                                                            .to_string(),
                                                    }) {
                                                        Ok(_) => {}
                                                        Err(err) => {
                                                            println!(
                                                                "Error while sending the notification to the receiver: {err}"
                                                            );
                                                        }
                                                    }
                                                    break;
                                                }
                                            };
                                            if let Some(s) = map.get(&to) {
                                                for tx in s.values() {
                                                    match tx.send(InternalMessage::Notification {
                                                        sender: from.clone(),
                                                        content: message.clone(),
                                                        resp_msg: Some(r_m.clone()),
                                                        resp_user: Some(r_u.clone()),
                                                    }) {
                                                        Ok(_) => {}
                                                        Err(err) => println!(
                                                            "Error while sending the notification to the receiver: {err}"
                                                        ),
                                                    };
                                                }
                                            }
                                        }

                                        match tx_clone.send(InternalMessage::Response {
                                            id,
                                            succes: r.succes,
                                            message: r.message,
                                        }) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                println!(
                                                    "Error while sending error to client: {err}"
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    false => {
                                        match tx_clone.send(InternalMessage::Response {
                                            id,
                                            succes: r.succes,
                                            message: r.message,
                                        }) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                println!(
                                                    "Error while sending error to client: {err}"
                                                );
                                                break;
                                            }
                                        }
                                    }
                                },
                                Err(err) => {
                                    println!("Error while working with the database: {err}");

                                    match tx_clone.send(InternalMessage::Response {
                                        id,
                                        succes: false,
                                        message: "Internal server error".to_string(),
                                    }) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!("Error while sending error to client: {err}");
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            match app_state.database.send_message(&from, &to, &message).await {
                                Ok(r) => match r.succes {
                                    true => {
                                        {
                                            let map = match app_state.map.lock() {
                                                Ok(m) => m,
                                                Err(err) => {
                                                    println!(
                                                        "Error while locking the map in app_state: {err}"
                                                    );
                                                    match tx_clone.send(InternalMessage::Response {
                                                        id,
                                                        succes: false,
                                                        message: "Internal server error"
                                                            .to_string(),
                                                    }) {
                                                        Ok(_) => {}
                                                        Err(err) => {
                                                            println!(
                                                                "Error while sending the notification to the receiver: {err}"
                                                            );
                                                        }
                                                    }
                                                    break;
                                                }
                                            };
                                            if let Some(s) = map.get(&to) {
                                                for tx in s.values() {
                                                    match tx.send(InternalMessage::Notification {
                                                        sender: from.clone(),
                                                        content: message.clone(),
                                                        resp_msg: None,
                                                        resp_user: None,
                                                    }) {
                                                        Ok(_) => {}
                                                        Err(err) => println!(
                                                            "Error while sending the notification to the receiver: {err}"
                                                        ),
                                                    };
                                                }
                                            }
                                        }

                                        match tx_clone.send(InternalMessage::Response {
                                            id,
                                            succes: r.succes,
                                            message: r.message,
                                        }) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                println!(
                                                    "Error while sending error to client: {err}"
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    false => {
                                        match tx_clone.send(InternalMessage::Response {
                                            id,
                                            succes: r.succes,
                                            message: r.message,
                                        }) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                println!(
                                                    "Error while sending error to client: {err}"
                                                );
                                                break;
                                            }
                                        }
                                    }
                                },
                                Err(err) => {
                                    println!("Error while working with the database: {err}");

                                    match tx_clone.send(InternalMessage::Response {
                                        id,
                                        succes: false,
                                        message: "Internal server error".to_string(),
                                    }) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!("Error while sending error to client: {err}");
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(WsMessage::GetMessage { from, idx }) => {
                        match app_state
                            .database
                            .get_messages(&session_info.username, &from, idx)
                            .await
                        {
                            Ok(Some(v)) => {
                                let mut chat_messages: Vec<(
                                    String,
                                    String,
                                    Option<String>,
                                    Option<String>,
                                )> = Vec::new();
                                for row in v {
                                    chat_messages.push((
                                        row.get(1),
                                        row.get(0),
                                        row.get(2),
                                        row.get(3),
                                    ));
                                }
                                match tx_clone.send(InternalMessage::Chat {
                                    messages: chat_messages,
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        println!("Error while sending error to client: {err}");
                                        break;
                                    }
                                }
                            }
                            Ok(None) => {}
                            Err(err) => {
                                println!("Error while getting the messages: {err}");
                            }
                        }
                    }
                    Ok(WsMessage::GetUserList { user }) => {
                        match app_state.database.get_user_list(&user).await {
                            Ok(Some(v)) => {
                                let mut users: Vec<String> = Vec::new();
                                for row in v {
                                    users.push(row.get(0));
                                }
                                match tx_clone.send(InternalMessage::Users { users_list: users }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        println!("Error while sending error to client: {err}");
                                        break;
                                    }
                                }
                            }
                            Ok(None) => {}
                            Err(err) => {
                                println!("Error while getting users: {err}");
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error: {err}");
                    }
                }
            }
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
        app_state.session_manager.close_session(&session_info.token);
    }
}
