use crate::network_manager::{
    database_manager::DataBase, handlers::Handlers, session_manager::SessionManager,
};
use axum::{
    Router,
    routing::{get, post, any},
};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    collections::HashMap,
    error::Error,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

pub enum InternalMessage {
    Chat { sender: String, content: String },
    Quit,
}

type UserTx = mpsc::UnboundedSender<InternalMessage>;
type UserSessions = HashMap<String, UserTx>;
pub struct AppState {
    pub session_manager: Arc<SessionManager>,
    pub database: Arc<DataBase>,
    pub map: Arc<Mutex<HashMap<String, UserSessions>>>,
}

pub struct Server {
    session_manager: Arc<SessionManager>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            session_manager: Arc::new(SessionManager::new()),
        }
    }
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let database = DataBase::new().await?;
        let app_state = Arc::new(AppState {
            session_manager: self.session_manager.clone(),
            database: database.clone(),
            map: Arc::new(Mutex::new(HashMap::new())),
        });

        let start_routes: Router = Router::new()
            .route("/login", get(Handlers::login))
            .route("/signin", post(Handlers::signin))
            .with_state(app_state.clone());
        let messenger_routes: Router = Router::new()
            .route("/messenger/ws", any(Handlers::ws_handler))
            .route("/messenger/send_message", post(Handlers::send_message))
            .with_state(app_state.clone());
        let app = Router::new().merge(start_routes).merge(messenger_routes);
        let config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("server.crt"),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("server.key"),
        )
        .await?;
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await?;
        Ok(())
    }
}
