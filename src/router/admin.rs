use crate::admin::content::router;
use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(router::init_router())
}
