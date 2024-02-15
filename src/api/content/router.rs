use axum::{
    routing::{get, post},
    Router,
};

use super::handler::hello;

pub fn init_router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello", post(hello))
}