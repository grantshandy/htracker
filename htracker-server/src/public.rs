use actix_web::{get, http::header::ContentType, HttpResponse};
use lazy_static::lazy_static;

const LOGIN_HTML: &'static str = include_str!("../web/login/login.html");
const LOGIN_JS: &'static str = include_str!("../web/login/login.js");

const REGISTER_HTML: &'static str = include_str!("../web/register/register.html");
const REGISTER_JS: &'static str = include_str!("../web/register/register.js");

const DASHBOARD_HTML: &'static str = include_str!("../web/dashboard/dashboard.html");
const DASHBOARD_JS: &'static str = include_str!("../web/dashboard/dashboard.js");

const INDEX_HTML: &'static str = include_str!("../web/index.html");
const INDEX_JS: &'static str = include_str!("../web/index.js");

const TAILWIND: &'static str = include_str!("../web/tailwind.css");

lazy_static! {
    static ref LOGIN: String = {
        LOGIN_HTML.replace("INSERT_SCRIPT", LOGIN_JS)
    };

    static ref REGISTER: String = {
        REGISTER_HTML.replace("INSERT_SCRIPT", REGISTER_JS)
    };

    static ref DASHBOARD: String = {
        DASHBOARD_HTML.replace("INSERT_SCRIPT", DASHBOARD_JS)
    };

    static ref INDEX: String = {
        INDEX_HTML.replace("INSERT_SCRIPT", INDEX_JS)
    };
}

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
