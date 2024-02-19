use axum::Router;

use wepress::router::{admin, api, site, static_file};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .nest("/", site::routes())
        .nest("/admin", admin::routes())
        .nest("/api", api::routes())
        .nest("/static", static_file::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
