use axum::{
    routing::{get, post},
    Router,
};

use super::handler::{create_user, hello, post_hello};

pub fn init_router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/user", get(create_user))
        .route("/hello", post(post_hello))
}
