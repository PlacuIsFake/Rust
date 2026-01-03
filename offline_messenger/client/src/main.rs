use reqwest::Certificate;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::mpsc::{Receiver, Sender, channel}};
use eframe::egui;

#[derive(Deserialize, Serialize, Clone)]
pub struct SigninReq {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    pub succes: bool,
    pub message: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResp {
    pub succes: bool,
    pub token: String,
    pub message: String,
}

pub enum Event {
    Login(String),
    Error(String)
}
enum Page {
    Login,
    MainApp
}

struct MyApp {
    current_page: Page,
    username: String,
    password: String,
    token: String,
    rx: Receiver<Event>,
    tx: Sender<Event>,
    client: reqwest::Client,
    err_msg: String
}

impl MyApp {
    fn new(client: reqwest::Client) -> Self {
        let (tx, rx) = channel();
        Self { current_page: Page::Login, username: String::new(), password: String::new(), token: String::new(), rx, tx, client, err_msg: String::new() }
    }
    fn show_login_screen(&mut self, ctx: &egui::Context)
    {
        if let Ok(event) = self.rx.try_recv() {
            match event {
                Event::Login(token) => {
                    self.token = token;
                    self.current_page = Page::MainApp;
                    ctx.request_repaint();
                }
                Event::Error(err) => {
                    self.err_msg = err;
                }
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
                    let login = LoginReq {
                        username: self.username.clone(),
                        password: self.password.clone()
                    };
                    let tx_clone = self.tx.clone();
                    let ctx_clone = ctx.clone();
                    let client_clone = self.client.clone();

                    tokio::spawn(async move {
                        let base_url = "https://127.0.0.1:3000";
                        let resp = match client_clone.get(format!("{base_url}/login")).json(&login).send().await {
                            Ok(snd ) => match snd.json::<LoginResp>().await {
                                Ok(r) => r,
                                Err(err) => {
                                    LoginResp { succes: false, token: "".to_string(), message: format!("Invalid response from the server after login: {err}") }
                                }
                            }
                            Err(err) => {
                                LoginResp { succes: false, token: "".to_string(), message: format!("Error while sending the login request: {err}") }
                            }
                        };

                        let result = match resp.succes {
                            true => Event::Login(resp.token),
                            false => Event::Error(resp.message),
                        };

                        let _ = tx_clone.send(result);
                        ctx_clone.request_repaint();
                        
                    });

                }

                if !self.err_msg.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.err_msg);
                }
            });
        });
    }
    fn show_main_app(&mut self, ctx: &egui::Context)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Something");
            ui.label(format!("Logged in as: {}", self.username));
            ui.add_space(20.0);
            ui.label(format!("{}", self.token));

            if ui.button("Log Out").clicked() {
                // Clear sensitive data
                self.password.clear();
                // Switch back to login
                self.current_page = Page::Login;
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.current_page {
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
        "My egui App",
        options,
        Box::new(|cc| {
            Ok(Box::<MyApp>::new(MyApp::new(client)))
        }),
    )

}
