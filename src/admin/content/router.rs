use super::handler::{home, post_hello};
use crate::global::template::APP_STATE;
use axum::{
    routing::{get, post},
    Router,
};

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(home))
        .with_state(APP_STATE.clone())
        .route("/post", post(post_hello))
}
