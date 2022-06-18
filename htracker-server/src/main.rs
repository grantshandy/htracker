use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use mongodb::{options::ClientOptions, Client, Database};

mod auth;
mod data;
mod email;
mod public;

pub const BASE_URL: &'static str = "http://localhost:8080";

#[derive(Clone)]
struct ServerData {
    pub db: Database,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    client_options.app_name = Some("htracker".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("htracker");
    let server_data = ServerData { db };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(server_data.clone())
            // these are all their individual services
            // instead of accessing a directory because
            // I want the binary to be self contained
            .service(public::login_html)
            .service(public::login_js)
            .service(public::dashboard_html)
            .service(public::dashboard_js)
            .service(public::index_html)
            .service(public::index_js)
            .service(public::tailwind_css)
            // auth is the authentication and user
            // management module of the server
            .service(auth::login_auth)
            .service(auth::register_account)
            .service(auth::verify_account)
            // these are the parts of the api
            // that involve accessing user data
            .service(data::add_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub fn bad_request_error(error: &str) -> HttpResponse {
    HttpResponse::BadRequest().body(format!("{{\"error\":\"{error}\"}}"))
}

pub fn server_error(error: &str) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!("{{\"error\":\"{error}\"}}"))
}
