use axum::Router;

use wepress::{router::api, router::admin, router::statics};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .nest("/admin", admin::routes())
        .nest("/api", api::routes())
        .nest("/static", statics::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
