use actix_web::{middleware::Logger, App, HttpServer};
use mongodb::{options::ClientOptions, Client, Database};

mod auth;
mod email;
mod public;

pub const BASE_URL: &'static str = "http://localhost:8080";

#[derive(Clone)]
struct ServerData {
    pub htracker_db: Database,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    client_options.app_name = Some("htracker".to_string());
    let client = Client::with_options(client_options).unwrap();
    let htracker_db = client.database("htracker");
    let server_data = ServerData { htracker_db };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(server_data.clone())
            .service(public::login_html)
            .service(public::login_js)
            .service(public::dashboard_html)
            .service(public::dashboard_js)
            .service(public::index_html)
            .service(public::index_js)
            .service(auth::login)
            .service(auth::register)
            .service(auth::verify)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
