use axum::{
    routing::{get, post},
    Router,
};
use axum_template::engine::Engine;
use tera::Tera;

use crate::template;

use super::handler::{handle_home, post_hello};

pub fn init_router(tera: Tera) -> Router {
    Router::new()
        .route("/home", get(handle_home))
        .with_state(template::AppState {
            engine: Engine::from(tera),
        })
        .route("/post", post(post_hello))
}
