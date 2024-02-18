use axum::{
    routing::{get, post},
    Router,
};
use axum_template::engine::Engine;
use tera::Tera;

use crate::template;

use super::handler::{login, login_action, ping_db, setup};

pub fn init_router(tera: Tera) -> Router {
    Router::new()
        .route("/auth/login", get(login).post(login_action))
        .route("/setup", get(setup))
        .with_state(template::AppState {
            engine: Engine::from(tera),
        })
        .route("/setup/ping_db", post(ping_db))
}