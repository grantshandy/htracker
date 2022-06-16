use actix_web::{
    get, http::header::ContentType, middleware::Logger, web, App, HttpResponse, HttpServer,
};

const INDEX_HTML: &'static str = include_str!("../static/index.html");
const INDEX_JS: &'static str = include_str!("../static/index.js");
const NOT_FOUND: &'static str = include_str!("../static/404.html");

#[get("/")]
async fn index_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(INDEX_HTML)
}

#[get("/index.js")]
async fn index_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(INDEX_JS)
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(NOT_FOUND)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("debug");

    HttpServer::new(|| {
        App::new()
            .service(index_html)
            .service(index_js)
            .default_service(web::route().to(not_found))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
