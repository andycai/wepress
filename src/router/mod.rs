use crate::admin::content::router as admin_router;
use crate::api::content::router as api_router;
use crate::api::site::router as site_router;

use axum::Router;
use tower_http::services::ServeDir;

pub fn routes() -> axum::Router {
    Router::new()
        .nest("/", site_router::init_router())
        .nest("/admin", admin_router::init_router())
        .nest("/api", api_router::init_router())
        .nest_service("/static", ServeDir::new("static"))
}
