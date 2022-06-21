use std::{fs::File, io::BufReader};

use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use argh::FromArgs;
use mongodb::{options::ClientOptions, Client, Database};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

mod auth;
mod data;
mod email;
mod public;

#[derive(FromArgs, Clone)]
/// Htracker server
struct HtrackerArgs {
    /// the url where this is being hosted
    #[argh(option)]
    base_url: String,
    /// ip address to serve on
    #[argh(option)]
    ip: String,
    /// port to serve on
    #[argh(option)]
    port: u16,
    /// cert to use
    #[argh(option)]
    cert: Option<String>,
    /// key to use
    #[argh(option)]
    key: Option<String>,
}

#[derive(Clone)]
struct ServerData {
    pub db: Database,
    pub args: HtrackerArgs,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init logger
    pretty_env_logger::try_init_custom_env("debug").unwrap();

    // init args
    let args: HtrackerArgs = argh::from_env();
    let ip = &args.ip;
    let port = &args.port;

    println!("starting http server on {ip}:{port}");

    // connect to mongodb
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    client_options.app_name = Some("htracker".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("htracker");

    // init server data
    let server_data = ServerData {
        db,
        args: args.clone(),
    };

    // start http server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(server_data.clone())
            // these are all their individual services
            // instead of accessing a directory because
            // I want the binary to be self contained
            // login page
            .service(public::login_html)
            .service(public::login_js)
            // register page
            .service(public::register_html)
            .service(public::register_js)
            // dashboard page
            .service(public::dashboard_html)
            .service(public::dashboard_js)
            // html page
            .service(public::index_html)
            .service(public::index_js)
            // generated tailwind css page
            .service(public::tailwind_css)
            // auth is the authentication and user
            // management module of the server
            .service(auth::auth)
            .service(auth::register_account)
            .service(auth::validate_account)
            // these are the parts of the api
            // that involve accessing user data
            .service(data::add_task)
            .service(data::remove_task)
            .service(data::get_tasks)
    });

    if let Some(cert) = &args.cert {
        println!("using cert at {cert}");
        if let Some(key) = &args.key {
            println!("using key at {key}");
            match server
                .bind_rustls(&format!("{ip}:{port}"), rustls_config(cert, key))?
                .run()
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("unable to start server, error: {err}");
                    std::process::exit(1);
                }
            }
        }
    } else {
        match server.bind(&format!("{ip}:{port}"))?.run().await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("unable to start server, error: {err}");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

pub fn bad_request_error(error: &str) -> HttpResponse {
    HttpResponse::BadRequest().body(format!("{{\"error\":\"{error}\"}}"))
}

pub fn server_error(error: &str) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!("{{\"error\":\"{error}\"}}"))
}

fn rustls_config<A: AsRef<str>>(cert: A, key: A) -> ServerConfig {
    let cert = cert.as_ref();
    let key = key.as_ref();

    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
