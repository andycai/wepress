use axum::response::IntoResponse;
use axum_template::RenderHtml;
use serde::Serialize;

use crate::template::AppEngine;

#[derive(Clone, Serialize)]
struct Site {
    favicon_url: String,
    logo_url: String,
    signup_url: String,
    signup_text: String,
    login_next: String,
    title: String,
    site_name: String,
    description: String,
    keywords: String,
    maincss_url: String,
    extra_css: String,
    extra_js: String,
    reset_password_url: String,
    user_id_type: String,
    copyright: String,
}

pub async fn login(engine: AppEngine) -> impl IntoResponse {
    let site = Site {
        signup_url:  "/auth/register".to_string(),
		signup_text:  "Sign up".to_string(),
		login_next:  "/admin".to_string(),
		site_name:    "Wepress".to_string(),
		logo_url:    "/static/img/logo.svg".to_string(),
		favicon_url: "/static/img/favicon.png".to_string(),
		title:       "Sign in".to_string(),
        maincss_url: "".to_string(),
        description: "Wepress is a lightweight headless CMS".to_string(),
        keywords: "cms, headless, wepress, blog, website, admin".to_string(),
        extra_css: "".to_string(),
        extra_js: "".to_string(),
        reset_password_url: "/auth/reset".to_string(),
        user_id_type: "email".to_string(),
        copyright: "2024 Wepress".to_string(),
    };
    RenderHtml("signin.html", engine, site)
}

pub async fn login_action() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn setup(engine: AppEngine) -> impl IntoResponse {
    "Hello, World!"
}

pub async fn ping_db() -> impl IntoResponse {
    "Hello, World!"
}