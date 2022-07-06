use actix_web::{get, http::header::ContentType, HttpResponse};

const CHUNK_VENDORS: &'static str = include_str!("../../frontend/dist/chunk-vendors.js");

const LOGIN_CSS: &'static str = include_str!("../../frontend/dist/login.css");
const LOGIN_HTML: &'static str = include_str!("../../frontend/dist/login.html");
const LOGIN_JS: &'static str = include_str!("../../frontend/dist/login.js");

const INDEX_CSS: &'static str = include_str!("../../frontend/dist/index.css");
const INDEX_HTML: &'static str = include_str!("../../frontend/dist/index.html");
const INDEX_JS: &'static str = include_str!("../../frontend/dist/index.js");

const REGISTER_CSS: &'static str = include_str!("../../frontend/dist/register.css");
const REGISTER_HTML: &'static str = include_str!("../../frontend/dist/register.html");
const REGISTER_JS: &'static str = include_str!("../../frontend/dist/register.js");

const DASHBOARD_CSS: &'static str = include_str!("../../frontend/dist/dashboard.css");
const DASHBOARD_HTML: &'static str = include_str!("../../frontend/dist/dashboard.html");
const DASHBOARD_JS: &'static str = include_str!("../../frontend/dist/dashboard.js");

#[get("/chunk-vendors.js")]
pub async fn chunk_vendors() -> HttpResponse {
    HttpResponse::Ok()
        // .content_type("application/javascript; charset=utf-8")
        .body(CHUNK_VENDORS)
}

#[get("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(LOGIN_HTML)
}

#[get("/login.css")]
pub async fn login_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(LOGIN_CSS)
}

#[get("/login.js")]
pub async fn login_js() -> HttpResponse {
    HttpResponse::Ok()
        // .content_type("application/javascript; charset=utf-8")
        .body(LOGIN_JS)
}

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(INDEX_HTML)
}

#[get("/index.css")]
pub async fn index_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(INDEX_CSS)
}

#[get("/index.js")]
pub async fn index_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(INDEX_JS)
}

#[get("/register")]
pub async fn register() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(REGISTER_HTML)
}

#[get("/register.css")]
pub async fn register_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(REGISTER_CSS)
}

#[get("/register.js")]
pub async fn register_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(REGISTER_JS)
}

#[get("/dashboard")]
pub async fn dashboard() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(DASHBOARD_HTML)
}

#[get("/dashboard.css")]
pub async fn dashboard_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(DASHBOARD_CSS)
}

#[get("/dashboard.js")]
pub async fn dashboard_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(DASHBOARD_JS)
}
