use crate::admin::content::router;

use axum::Router;
use tera::Tera;

pub fn routes(tera: Tera) -> Router {
    Router::new()
        .merge(router::init_router(tera))
}