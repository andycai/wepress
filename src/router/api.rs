use axum::Router;

use crate::api::content::router;

pub fn routes() -> Router {
    Router::new()
        .merge(router::init_router())
}