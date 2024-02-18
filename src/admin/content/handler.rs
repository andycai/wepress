use axum::{http::StatusCode, response::IntoResponse, Json};

use axum_template::RenderHtml;
use serde::{Deserialize, Serialize};

use crate::template::AppEngine;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn handle_home(engine: AppEngine) -> impl IntoResponse {
    let user = User {
        id: 8888,
        username: "Andy".to_string(),
    };

    RenderHtml("index.html", engine, user)
}

pub async fn handle_hello() -> impl IntoResponse {
    "Admin Hello, world!"
}

pub async fn post_hello() -> impl IntoResponse {
    "Admin Hello, world!"
}

pub async fn handle_create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
