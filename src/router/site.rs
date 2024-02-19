use crate::api::site::router;
use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(router::init_router())
}
