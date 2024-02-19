use dotenv::dotenv;
use wepress::{global, router};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = router::routes();

    dotenv().ok();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(global::conf::ADDR.clone())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
