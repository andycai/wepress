use crate::admin::content::router as admin_router;
use crate::api::content::router as api_router;
use crate::api::site::router as site_router;

use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub fn routes() -> axum::Router {
    Router::new()
        .nest("/", site_router::init_router())
        .nest("/admin", admin_router::init_router())
        .nest("/api", api_router::init_router())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ]),
        )
        .nest_service("/static", ServeDir::new("static"))
}
