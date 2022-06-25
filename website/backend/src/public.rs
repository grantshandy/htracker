use actix_web::{get, http::header::ContentType, HttpResponse};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGIN: String = include_str!(BASE_WEB_DIR + "/login.html")
        .replace("INSERT_SCRIPT", include_str!("../web/login/login.js"));
    static ref REGISTER: String = include_str!("../web/register/register.html")
        .replace("INSERT_SCRIPT", include_str!("../web/register/register.js"));
    static ref DASHBOARD: String = include_str!("../web/dashboard/dashboard.html").replace(
        "INSERT_SCRIPT",
        include_str!("../web/dashboard/dashboard.js")
    );
    static ref INDEX: String =
        include_str!("../web/index.html").replace("INSERT_SCRIPT", include_str!("../web/index.js"));
}

const TAILWIND: &'static str = include_str!("../web/tailwind.css");

#[get("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(LOGIN.as_str())
}

#[get("/register")]
pub async fn register() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(REGISTER.as_str())
}

#[get("/dashboard")]
pub async fn dashboard() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(DASHBOARD.as_str())
}

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(INDEX.as_str())
}

#[get("/tailwind.css")]
pub async fn tailwind() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(TAILWIND)
}
