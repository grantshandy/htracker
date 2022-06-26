use std::{fs::File, io::BufReader, time::Duration};

use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use argh::FromArgs;
use mongodb::{options::ClientOptions, Client, Database};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

mod auth;
mod data;
mod email;
mod public;
mod quote;

#[derive(FromArgs, Clone)]
/// Htracker server
struct HtrackerArgs {
    /// the url where this is being hosted
    #[argh(option)]
    base_url: String,
    /// ip address to serve on
    #[argh(option)]
    ip: String,
    /// http port to serve on
    #[argh(option)]
    http_port: u16,
    /// https port to serve on
    #[argh(option)]
    https_port: Option<u16>,
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
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(err) => {
            eprintln!("htracker failed to execute");
            println!("Error:\n{:?}", err);
            std::process::exit(1);
        }
    }
}

async fn run() -> std::io::Result<()> {
    // init logger
    pretty_env_logger::init_custom_env("info");

    // init args and port for http
    let args: HtrackerArgs = argh::from_env();
    let ip = &args.ip;

    let http_serve_str = format!("{}:{}", ip, args.http_port);
    println!("starting http on {http_serve_str}");

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
    let mut server = HttpServer::new(move || {
        // init rate limiter
        let rate_limit_backend = InMemoryBackend::builder().build();
        let rate_limit_input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 50)
            .real_ip_key()
            .build();

        let rate_limiter = RateLimiter::builder(rate_limit_backend, rate_limit_input)
            .add_headers()
            .build();

        App::new()
            .wrap(Logger::default())
            .wrap(rate_limiter)
            .app_data(server_data.clone())
            // these are all their individual services
            // instead of accessing a directory because
            // I want the binary to be self contained
            .service(public::chunk_vendors)
            .service(public::login)
            .service(public::login_css)
            .service(public::login_js)
            .service(public::index)
            .service(public::index_css)
            .service(public::index_js)
            .service(public::register)
            .service(public::register_css)
            .service(public::register_js)
            // serve quotes
            .service(quote::quote)
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
    })
    .bind(http_serve_str)?;

    // init https if required
    // hopefully rust will eventually figure out the 'if let Enum' chaining bullshit in a new release
    if let Some(cert) = &args.cert {
        println!("using cert at {cert}");
        if let Some(key) = &args.key {
            println!("using key at {key}");
            if let Some(https_port) = &args.https_port {
                let https_serve_str = format!("{ip}:{https_port}");
                println!("serving https on {https_serve_str}");
                server = server.bind_rustls(&https_serve_str, rustls_config(cert, key))?;
            }
        }
    }

    // run the server
    server.run().await?;

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
    let cert_file = &mut BufReader::new(File::open(cert).expect("couldn't find cert"));
    let key_file = &mut BufReader::new(File::open(key).expect("couldn't find key"));

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
