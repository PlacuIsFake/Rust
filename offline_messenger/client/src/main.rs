use core::f32;
use eframe::egui;
use futures_util::{SinkExt, StreamExt};
use reqwest::Certificate;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
};
use tokio_tungstenite::connect_async_tls_with_config;

#[derive(Deserialize, Serialize, Clone)]
struct SigninReq {
    username: String,
    password: String,
}
#[derive(Deserialize, Serialize, Clone)]
struct LoginReq {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct SessionInfo {
    username: String,
    token: String,
}
#[derive(Deserialize, Serialize, Clone)]
struct ChatMessage {
    id: String,
    from: String,
    to: String,
    message: String,
    resp_msg: Option<String>,
    resp_user: Option<String>,
}
#[derive(Clone, PartialEq)]
enum MessageStatus {
    Sending,
    Sent,
    Failed,
}
struct OnScreenMessage {
    id: String,
    from: String,
    message: String,
    resp_msg: Option<String>,
    resp_usr: Option<String>,
    status: MessageStatus,
}
#[derive(Serialize, Deserialize, Clone)]
struct Response {
    succes: bool,
    message: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct LoginResp {
    succes: bool,
    token: String,
    message: String,
}

enum LoginEvent {
    Signin,
    Login(String),
    Error(String),
    ServerResponse((String, bool, String)),
    ChatDump(Vec<(String, String, Option<String>, Option<String>)>),
    NewMessage(ChatMessage),
    TheList(Vec<String>),
    ConnectionLost,
}
enum Event {
    NewMessage(ChatMessage),
    ChangeChat((String, i64)),
    GetUsersList(String),
}
enum Page {
    Signin,
    Login,
    MainApp,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
enum WsMessage {
    SendMessage {
        id: String,
        token: String,
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
        to: String,
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

fn start_websocket(
    session_info: SessionInfo,
    ctx: egui::Context,
    gui_sender: Sender<LoginEvent>,
) -> Option<tokio::sync::mpsc::Sender<Event>> {
    let (gui_msg_tx, mut gui_msg_rx) = tokio::sync::mpsc::channel::<Event>(32);
    let session_info_clone = session_info.clone();
    tokio::spawn(async move {
        let url = "wss://127.0.0.1:3000/ws";
        let connector = match native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
        {
            Ok(c) => c,
            Err(err) => {
                println!("Error while building connector: {err}");
                return;
            }
        };

        let conn = tokio_tungstenite::Connector::NativeTls(connector);

        match connect_async_tls_with_config(url, None, false, Some(conn)).await {
            Ok((ws_stream, _)) => {
                let (mut wr, mut rd) = ws_stream.split();
                tokio::spawn(async move {
                    if let Ok(msg_back) = serde_json::to_string(&session_info) {
                        let _ = wr
                            .send(tokio_tungstenite::tungstenite::Message::Text(
                                msg_back.into(),
                            ))
                            .await;
                    }

                    while let Some(msg) = gui_msg_rx.recv().await {
                        match msg {
                            Event::NewMessage(c) => {
                                let ceva = WsMessage::SendMessage {
                                    id: c.id,
                                    token: session_info_clone.token.clone(),
                                    from: c.from,
                                    to: c.to,
                                    message: c.message,
                                    resp_msg: c.resp_msg,
                                    resp_user: c.resp_user,
                                };
                                if let Ok(msg_back) = serde_json::to_string(&ceva) {
                                    let _ = wr
                                        .send(tokio_tungstenite::tungstenite::Message::Text(
                                            msg_back.into(),
                                        ))
                                        .await;
                                }
                            }
                            Event::ChangeChat(c) => {
                                let ceva = WsMessage::GetMessage {
                                    from: c.0,
                                    idx: c.1,
                                };
                                if let Ok(msg_back) = serde_json::to_string(&ceva) {
                                    let _ = wr
                                        .send(tokio_tungstenite::tungstenite::Message::Text(
                                            msg_back.into(),
                                        ))
                                        .await;
                                }
                            }
                            Event::GetUsersList(user) => {
                                let ceva = WsMessage::GetUserList { user };
                                if let Ok(msg_back) = serde_json::to_string(&ceva) {
                                    let _ = wr
                                        .send(tokio_tungstenite::tungstenite::Message::Text(
                                            msg_back.into(),
                                        ))
                                        .await;
                                }
                            }
                        }
                    }
                });

                while let Some(Ok(msg)) = rd.next().await {
                    if let tokio_tungstenite::tungstenite::Message::Text(raw_json) = msg {
                        let message: Result<WsMessageBack, _> = serde_json::from_str(&raw_json);
                        match message {
                            Ok(WsMessageBack::Message {
                                from: msg_from,
                                to: msg_to,
                                message: msg_content,
                                resp_msg: r_m,
                                resp_user: r_u,
                            }) => {
                                let rand_id = format!("{}", uuid::Uuid::new_v4());
                                let _ = gui_sender.send(LoginEvent::NewMessage(ChatMessage {
                                    id: rand_id,
                                    from: msg_from,
                                    to: msg_to,
                                    message: msg_content,
                                    resp_msg: r_m,
                                    resp_user: r_u,
                                }));
                            }
                            Ok(WsMessageBack::Response {
                                id,
                                succes,
                                message,
                            }) => {
                                let _ = gui_sender
                                    .send(LoginEvent::ServerResponse((id, succes, message)));
                            }
                            Ok(WsMessageBack::Chat { messages }) => {
                                let _ = gui_sender.send(LoginEvent::ChatDump(messages));
                            }
                            Ok(WsMessageBack::UserList { list }) => {
                                let _ = gui_sender.send(LoginEvent::TheList(list));
                            }
                            Err(err) => {
                                println!("Error while recieving message from server: {err}");
                            }
                        }
                        ctx.request_repaint();
                    }
                }
                println!("Connection ended");
                let _ = gui_sender.send(LoginEvent::ConnectionLost);
            }
            Err(err) => {
                println!("Connection failed: {err}");
            }
        };
    });
    Some(gui_msg_tx)
}

struct MyApp {
    current_page: Page,
    username: String,
    password: String,
    password_again: String,
    token: String,

    rx: Receiver<LoginEvent>,
    tx: Sender<LoginEvent>,
    client: reqwest::Client,

    chat: Vec<OnScreenMessage>,
    current_chat: String,
    message_input: String,
    selected_message: Option<String>,
    selected_from: Option<String>,

    contacts: Vec<String>,

    ws_tx: Option<tokio::sync::mpsc::Sender<Event>>,

    err_msg: String,
}

impl MyApp {
    fn new(client: reqwest::Client) -> Self {
        let (tx, rx) = channel();
        Self {
            current_page: Page::Login,
            username: String::new(),
            password: String::new(),
            password_again: String::new(),
            token: String::new(),
            rx,
            tx,
            client,
            chat: Vec::new(),
            current_chat: String::new(),
            message_input: String::new(),
            selected_message: None,
            selected_from: None,
            contacts: Vec::new(),
            ws_tx: None,
            err_msg: String::new(),
        }
    }
    fn show_signin_screen(&mut self, ctx: &egui::Context) {
        if let Ok(event) = self.rx.try_recv() {
            match event {
                LoginEvent::Signin => {
                    self.current_page = Page::Login;
                    self.username.clear();
                    self.password.clear();
                    self.password_again.clear();
                    self.err_msg.clear();
                    ctx.request_repaint();
                }
                LoginEvent::Error(err) => {
                    self.err_msg = err;
                }
                _ => {}
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("Rustcrab");
                ui.add_space(20.0);

                ui.label("Username");
                ui.text_edit_singleline(&mut self.username);
                ui.add_space(10.0);
                ui.label("Password");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                ui.add_space(10.0);

                ui.label("Password again");
                ui.add(egui::TextEdit::singleline(&mut self.password_again).password(true));
                ui.add_space(20.0);

                if ui.button("Sign in").clicked() {
                    if !self.username.trim().is_empty()
                    {
                        if !self.password.trim().is_empty()
                        {
                            if self.password == self.password_again {
                                let signin = SigninReq {
                                    username: self.username.clone(),
                                    password: self.password.clone(),
                                };
                                let tx_clone = self.tx.clone();
                                let ctx_clone = ctx.clone();
                                let client_clone = self.client.clone();

                                tokio::spawn(async move {
                                    let base_url = "https://127.0.0.1:3000";
                                    let resp = match client_clone
                                        .get(format!("{base_url}/signin"))
                                        .json(&signin)
                                        .send()
                                        .await
                                    {
                                        Ok(snd) => match snd.json::<Response>().await {
                                            Ok(r) => r,
                                            Err(err) => Response {
                                                succes: false,
                                                message: format!(
                                                    "Invalid response from the server after login: {err}"
                                                ),
                                            },
                                        },
                                        Err(err) => Response {
                                            succes: false,
                                            message: format!(
                                                "Error while sending the login request: {err}"
                                            ),
                                        },
                                    };

                                    let result = match resp.succes {
                                        true => LoginEvent::Signin,
                                        false => LoginEvent::Error(resp.message),
                                    };

                                    let _ = tx_clone.send(result);
                                    ctx_clone.request_repaint();
                                });
                            } else {
                                self.err_msg.clear();
                                self.err_msg = "The two passwords are not identical".to_string();
                                ctx.request_repaint();
                            }
                        } else {
                            self.err_msg.clear();
                            self.err_msg = "Please insert password".to_string();
                            ctx.request_repaint();
                        }
                    } else {
                        self.err_msg.clear();
                        self.err_msg = "Please insert username".to_string();
                        ctx.request_repaint();
                    }
                }

                if !self.err_msg.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.err_msg);
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label("Already have an account?");
                if ui.button("Log in").clicked() {
                    self.current_page = Page::Login;
                    self.err_msg.clear();
                    self.username.clear();
                    self.password.clear();
                    self.password_again.clear();
                }
            });
        });
    }
    fn show_login_screen(&mut self, ctx: &egui::Context) {
        if let Ok(event) = self.rx.try_recv() {
            match event {
                LoginEvent::Login(token) => {
                    self.token = token;
                    self.current_page = Page::MainApp;

                    let tx = start_websocket(
                        SessionInfo {
                            username: self.username.clone(),
                            token: self.token.clone(),
                        },
                        ctx.clone(),
                        self.tx.clone(),
                    );
                    self.ws_tx = tx;
                    ctx.request_repaint();
                }
                LoginEvent::Error(err) => {
                    self.err_msg = err;
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("RustCrab");
                ui.add_space(20.0);

                ui.label("Username");
                ui.text_edit_singleline(&mut self.username);
                ui.add_space(10.0);
                ui.label("Password");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                ui.add_space(20.0);

                if ui.button("Log In").clicked() {
                    if !self.username.trim().is_empty()
                    {
                        if !self.password.trim().is_empty()
                        {
                            let login = LoginReq {
                                username: self.username.clone(),
                                password: self.password.clone(),
                            };
                            let tx_clone = self.tx.clone();
                            let ctx_clone = ctx.clone();
                            let client_clone = self.client.clone();

                            tokio::spawn(async move {
                                let base_url = "https://127.0.0.1:3000";
                                let resp = match client_clone
                                    .get(format!("{base_url}/login"))
                                    .json(&login)
                                    .send()
                                    .await
                                {
                                    Ok(snd) => match snd.json::<LoginResp>().await {
                                        Ok(r) => r,
                                        Err(err) => LoginResp {
                                            succes: false,
                                            token: "".to_string(),
                                            message: format!(
                                                "Invalid response from the server after login: {err}"
                                            ),
                                        },
                                    },
                                    Err(err) => LoginResp {
                                        succes: false,
                                        token: "".to_string(),
                                        message: format!("Error while sending the login request: {err}"),
                                    },
                                };

                                let result = match resp.succes {
                                    true => LoginEvent::Login(resp.token),
                                    false => LoginEvent::Error(resp.message),
                                };

                                let _ = tx_clone.send(result);
                                ctx_clone.request_repaint();
                            });
                        } else {
                            self.err_msg.clear();
                            self.err_msg = "Please insert password".to_string();
                        }
                    } else {
                        self.err_msg.clear();
                        self.err_msg = "Please insert username".to_string();
                    }
                }

                if !self.err_msg.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.err_msg);
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label("Don't have an account?");
                if ui.button("Sign up").clicked() {
                    self.current_page = Page::Signin;
                    self.err_msg.clear();
                    self.username.clear();
                    self.password.clear();
                }
            });
        });
    }
    fn show_main_app(&mut self, ctx: &egui::Context) {
        if let Some(tx) = &self.ws_tx {
            let event = Event::GetUsersList(self.username.clone());
            let _ = tx.try_send(event);
        }
        while let Ok(event) = self.rx.try_recv() {
            match event {
                LoginEvent::ServerResponse((id, success, message)) => {
                    if let Some(msg) = self.chat.iter_mut().find(|m| m.id == id) {
                        if success {
                            msg.status = MessageStatus::Sent;
                        } else {
                            msg.status = MessageStatus::Failed;
                            println!("Message {id} failed: {message}");
                        }
                    }
                }
                LoginEvent::ChatDump(messages) => {
                    self.chat.clear();
                    for e in messages {
                        let rand_id = format!("{}", uuid::Uuid::new_v4());
                        self.chat.push(OnScreenMessage {
                            id: rand_id,
                            from: e.0,
                            message: e.1,
                            status: MessageStatus::Sent,
                            resp_msg: e.2,
                            resp_usr: e.3,
                        });
                    }
                }
                LoginEvent::NewMessage(c) => {
                    if c.from == self.current_chat || c.to == self.current_chat {
                        self.chat.push(OnScreenMessage {
                            id: c.id,
                            from: c.from,
                            message: c.message,
                            resp_msg: c.resp_msg,
                            resp_usr: c.resp_user,
                            status: MessageStatus::Sent,
                        });
                    }
                }
                LoginEvent::TheList(list) => {
                    self.contacts = list;
                }
                LoginEvent::ConnectionLost => {
                    self.current_page = Page::Login;
                    self.token.clear();
                    self.username.clear();
                    self.password.clear();
                    self.chat.clear();
                    self.contacts.clear();
                    self.current_chat.clear();
                    self.message_input.clear();
                    self.selected_message = None;
                    self.selected_from = None;
                    self.ws_tx = None;
                    self.err_msg = "Server unreacheble".to_string();
                }
                _ => {}
            }
        }
        egui::SidePanel::left("users_panel").show(ctx, |ui| {
            ui.heading("Contacts");
            ui.separator();

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(165, 42, 0);
                        ui.visuals_mut().selection.stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
                        for contact in &self.contacts {
                            let selected = *contact == self.current_chat;
                            if ui.selectable_label(selected, contact).clicked() && !selected {
                                self.current_chat = contact.to_string();
                                self.chat.clear();
                                self.message_input.clear();
                                self.selected_message = None;
                                self.selected_from = None;

                                if let Some(tx) = &self.ws_tx {
                                    let event = Event::ChangeChat((contact.to_string(), 0));
                                    let _ = tx.try_send(event);
                                }
                            }
                        }
                    });
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        ui.add_space(10.0);

                        if ui.button("Log Out").clicked() {
                            self.current_page = Page::Login;
                            self.token.clear();
                            self.username.clear();
                            self.password.clear();
                            self.chat.clear();
                            self.contacts.clear();
                            self.current_chat.clear();
                            self.message_input.clear();
                            self.selected_message = None;
                            self.selected_from = None;
                            self.ws_tx = None;
                        }
                    });
                },
            );
        });

        egui::TopBottomPanel::bottom("input_panel").show(ctx, |ui| {
            if let (Some(m), Some(u)) = (self.selected_message.clone(), self.selected_from.clone())
            {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    ui.label(egui::RichText::new("Replying to").color(egui::Color32::LIGHT_BLUE));
                    ui.label(egui::RichText::new(format!("{u}:")).strong());
                    if m.len() > 30 {
                        ui.label(
                            egui::RichText::new(format!("{}...", &m[..30]))
                                .italics()
                                .color(egui::Color32::GRAY),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(m.clone())
                                .italics()
                                .color(egui::Color32::GRAY),
                        );
                    }
                    if ui.small_button("X").clicked() {
                        self.selected_message = None;
                        self.selected_from = None;
                    }
                });
            }

            ui.horizontal(|ui| {
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.message_input)
                        .desired_width(f32::INFINITY)
                        .hint_text("Type a message..."),
                );
                if (ui.button("Send").clicked()
                    || (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                    && !self.message_input.trim().is_empty()
                    && !self.current_chat.trim().is_empty()
                {
                    let rand_id = format!("{}", uuid::Uuid::new_v4());

                    self.chat.push(OnScreenMessage {
                        id: rand_id.clone(),
                        from: self.username.clone(),
                        message: self.message_input.clone(),
                        status: MessageStatus::Sending,
                        resp_msg: self.selected_message.clone(),
                        resp_usr: self.selected_from.clone(),
                    });

                    if let Some(tx) = &self.ws_tx {
                        let event = Event::NewMessage(ChatMessage {
                            id: rand_id,
                            from: self.username.clone(),
                            to: self.current_chat.clone(),
                            message: self.message_input.clone(),
                            resp_msg: self.selected_message.clone(),
                            resp_user: self.selected_from.clone(),
                        });
                        let _ = tx.try_send(event);
                    }
                    self.message_input.clear();
                    self.selected_message = None;
                    self.selected_from = None;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;

                    for msg in &self.chat {
                        if msg.from == self.username {
                            let (bg_color, text_color) = match msg.status {
                                MessageStatus::Sending => {
                                    (egui::Color32::from_rgb(165, 42, 0), egui::Color32::GRAY)
                                }
                                MessageStatus::Sent => {
                                    (egui::Color32::from_rgb(165, 42, 0), egui::Color32::WHITE)
                                }
                                MessageStatus::Failed => (egui::Color32::RED, egui::Color32::WHITE),
                            };
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                let bubble = egui::Frame::none()
                                    .fill(bg_color)
                                    .rounding(egui::Rounding::same(15.0))
                                    .inner_margin(10.0);
                                bubble.show(ui, |ui| {
                                    ui.set_max_width(300.0);
                                    ui.vertical(|ui| {
                                        if let (Some(m), Some(u)) =
                                            (msg.resp_msg.clone(), msg.resp_usr.clone())
                                        {
                                            ui.label(
                                                egui::RichText::new(format!("Replying to {u}"))
                                                    .size(10.0)
                                                    .italics()
                                                    .color(
                                                        egui::Color32::LIGHT_GRAY
                                                            .gamma_multiply(0.8),
                                                    ),
                                            );
                                            ui.label(
                                                egui::RichText::new(&m).size(10.0).italics().color(
                                                    egui::Color32::LIGHT_GRAY.gamma_multiply(0.8),
                                                ),
                                            );
                                            ui.add_space(5.0);
                                            //ui.separator();
                                        }
                                        ui.label(
                                            egui::RichText::new(&msg.message).color(text_color),
                                        );
                                    });
                                });

                                if ui.small_button("↩").clicked() {
                                    self.selected_message = Some(msg.message.clone());
                                    self.selected_from = Some(msg.from.clone());
                                }
                            });
                        } else {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                                let bubble = egui::Frame::none()
                                    .fill(egui::Color32::from_rgb(62, 28, 28))
                                    .rounding(egui::Rounding::same(15.0))
                                    .inner_margin(10.0);
                                bubble.show(ui, |ui| {
                                    ui.set_max_width(300.0);
                                    ui.vertical(|ui| {
                                        if let (Some(m), Some(u)) = (&msg.resp_msg, &msg.resp_usr) {
                                            ui.label(
                                                egui::RichText::new(format!("Replying to {u}"))
                                                    .size(10.0)
                                                    .italics()
                                                    .color(
                                                        egui::Color32::LIGHT_GRAY
                                                            .gamma_multiply(0.8),
                                                    ),
                                            );
                                            ui.label(
                                                egui::RichText::new(m).size(10.0).italics().color(
                                                    egui::Color32::LIGHT_GRAY.gamma_multiply(0.8),
                                                ),
                                            );
                                            ui.add_space(5.0);
                                            //ui.separator();
                                        }
                                        ui.label(
                                            egui::RichText::new(&msg.message)
                                                .color(egui::Color32::WHITE),
                                        );
                                    });
                                });

                                if ui.small_button("↩").clicked() {
                                    self.selected_message = Some(msg.message.clone());
                                    self.selected_from = Some(msg.from.clone());
                                }
                            });
                        }
                    }
                });
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_page {
            Page::Signin => self.show_signin_screen(ctx),
            Page::Login => self.show_login_screen(ctx),
            Page::MainApp => self.show_main_app(ctx),
        }
    }
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let cert_bytes = match fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("server.crt"),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("Error while trying to read the certificate: {err}");
            return Ok(());
        }
    };
    let cert = match Certificate::from_pem(&cert_bytes) {
        Ok(c) => c,
        Err(err) => {
            println!("Invalid certificate: {err}");
            return Ok(());
        }
    };
    let client = match reqwest::Client::builder()
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(cli) => cli,
        Err(err) => {
            println!("Error while building the client: {err}");
            return Ok(());
        }
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustcrab",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp::new(client)))),
    )
}
