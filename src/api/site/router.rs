use super::handler::{login, login_action, ping_db, setup};
use crate::global::template::APP_STATE;
use axum::{
    routing::{get, post},
    Router,
};

pub fn init_router() -> Router {
    Router::new()
        .route("/auth/login", get(login).post(login_action))
        .route("/setup", get(setup))
        .with_state(APP_STATE.clone())
        .route("/setup/ping_db", post(ping_db))
}
