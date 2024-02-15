use axum::response::IntoResponse;

pub async fn hello() -> impl IntoResponse {
    "Hello, world!"
}