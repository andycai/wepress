use axum::Router;

use crate::admin::content::router;


pub fn routes() -> Router {
    Router::new()
        .merge(router::init_router())
}