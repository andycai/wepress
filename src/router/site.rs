use axum::Router;
use tera::Tera;

use crate::api::site::router;

pub fn routes(tera: Tera) -> Router {
    Router::new()
        .merge(router::init_router(tera))
}