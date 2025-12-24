use crate::network_manager::{
    database_manager::DataBase, handlers::Handlers, session_manager::SessionManager,
};
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

pub struct AppState {
    pub session_manager: Arc<SessionManager>,
    pub database: Arc<DataBase>,
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
        });

        let start_routes: Router = Router::new()
            .route("/login", get(Handlers::login))
            .route("/signin", post(Handlers::signin))
            .with_state(app_state.clone());

        Ok(())
    }
}
