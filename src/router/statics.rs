use axum::Router;

use tower_http::services::ServeDir;

pub fn routes() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("static"))
}