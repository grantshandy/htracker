use actix_web::{get, http::header::ContentType, HttpResponse};

const LOGIN_HTML: &'static str = include_str!("../public/login/login.html");
const LOGIN_JS: &'static str = include_str!("../public/login/login.js");

const DASHBOARD_HTML: &'static str = include_str!("../public/dashboard/dashboard.html");
const DASHBOARD_JS: &'static str = include_str!("../public/dashboard/dashboard.js");

const INDEX_HTML: &'static str = include_str!("../public/index.html");
const INDEX_JS: &'static str = include_str!("../public/index.js");

#[get("/login")]
pub async fn login_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(LOGIN_HTML)
}

#[get("/login.js")]
pub async fn login_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("Content-Type: application/javascript; charset=utf-8")
        .body(LOGIN_JS)
}

#[get("/dashboard")]
pub async fn dashboard_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(DASHBOARD_HTML)
}

#[get("/dashboard.js")]
pub async fn dashboard_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("Content-Type: application/javascript; charset=utf-8")
        .body(DASHBOARD_JS)
}

#[get("/")]
pub async fn index_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(INDEX_HTML)
}

#[get("/index.js")]
pub async fn index_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("Content-Type: application/javascript; charset=utf-8")
        .body(INDEX_JS)
}