use axum::{
    routing::{get, post},
    Router,
};
use axum_template::engine::Engine;
use tera::Tera;

use crate::template;

use super::handler::{home, post_hello};

pub fn init_router(tera: Tera) -> Router {
    Router::new()
        .route("/", get(home))
        .with_state(template::AppState {
            engine: Engine::from(tera),
        })
        .route("/post", post(post_hello))
}
