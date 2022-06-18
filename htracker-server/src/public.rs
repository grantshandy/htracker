use actix_web::{get, http::header::ContentType, HttpResponse};

const LOGIN_HTML: &'static str = include_str!("../dist/login/login.html");
const LOGIN_JS: &'static str = include_str!("../dist/login/login.js");

const REGISTER_HTML: &'static str = include_str!("../dist/register/register.html");
const REGISTER_JS: &'static str = include_str!("../dist/register/register.js");

const DASHBOARD_HTML: &'static str = include_str!("../dist/dashboard/dashboard.html");
const DASHBOARD_JS: &'static str = include_str!("../dist/dashboard/dashboard.js");

const INDEX_HTML: &'static str = include_str!("../dist/index.html");
const INDEX_JS: &'static str = include_str!("../dist/index.js");
const TAILWIND_CSS: &'static str = include_str!("../dist/tailwind.css");

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

#[get("/register")]
pub async fn register_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(REGISTER_HTML)
}

#[get("/register.js")]
pub async fn register_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("Content-Type: application/javascript; charset=utf-8")
        .body(REGISTER_JS)
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

#[get("/tailwind.css")]
pub async fn tailwind_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(TAILWIND_CSS)
}
