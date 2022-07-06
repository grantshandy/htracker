mod register;

use actix_web::{get, HttpRequest, HttpResponse};
use blake2::{Blake2b512, Digest};
use mongodb::bson::doc;
use rand::{distributions::Alphanumeric, Rng};
pub use register::{register_account, validate_account, NewUser};
use serde::{Deserialize, Serialize};

use crate::{bad_request_error, server_error, tasks::Task, ServerData};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub session_tokens: Vec<String>,
    pub data: Data,
}

impl User {
    pub fn from_new(new: &NewUser) -> Self {
        Self {
            username: new.username.clone(),
            password: hash_from_password(&new.password),
            email: new.email.clone(),
            session_tokens: Vec::new(),
            data: Data::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tasks: Vec<Task>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }
}

pub fn hash_from_password<A: AsRef<str>>(pass: A) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(pass.as_ref().as_bytes());

    base64::encode(hasher.finalize())
}

pub fn gen_session_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect()
}

pub async fn validate_session_token(req: &HttpRequest) -> Result<Option<String>, HttpResponse> {
    let server_data: &ServerData = req.app_data().unwrap();
    let users = &server_data.db.collection::<User>("users");

    let session_token = match req.headers().get("X-SessionToken") {
        Some(bytes) => match String::from_utf8(bytes.as_bytes().to_vec()) {
            Ok(session_token) => session_token,
            Err(_) => return Err(bad_request_error("session token not formatted in utf8")),
        },
        None => return Err(bad_request_error("must include token")),
    };

    println!("{session_token}");

    match users
        .find_one(doc! { "session_tokens": &session_token }, None)
        .await {
            Ok(user_data) => match user_data {
                Some(_) => return Ok(Some(session_token)),
                None => return Ok(None),
            },
            Err(error) => Err(server_error(&format!("internal server error: {error}"))),
        }
}

pub async fn get_user_data<A: AsRef<str>>(req: &HttpRequest, session_token: A) -> Result<Data, HttpResponse> {
    let server_data: &ServerData = req.app_data().unwrap();
    let users = &server_data.db.collection::<User>("users");

    match users
        .find_one(doc! { "session_tokens": session_token.as_ref() }, None)
        .await {
            Ok(user_data) => match user_data {
                Some(user_data) => return Ok(user_data.data),
                None => return Err(server_error("couldn't retrieve user data from session token")),
            },
            Err(error) => Err(server_error(&format!("internal server error: {error}"))),
        }
}

#[get("/api/login")]
pub async fn login(req: HttpRequest) -> HttpResponse {
    // get user and validate login token in header
    // get token bytes from header
    // formated username:password in base64
    let auth_token_bytes = match req.headers().get("X-AuthToken") {
        Some(token) => token,
        None => return bad_request_error("must include token"),
    };

    // base64 token in utf8
    let auth_token_base64 = match String::from_utf8(auth_token_bytes.as_bytes().to_vec()) {
        Ok(data) => data,
        Err(_) => return bad_request_error("couldn't turn encoded token into utf8"),
    };

    let auth_token_str = match base64::decode(auth_token_base64) {
        Ok(auth_token) => match String::from_utf8(auth_token) {
            Ok(auth_token) => auth_token,
            Err(_) => return bad_request_error("token not utf8"),
        },
        Err(_) => return bad_request_error("token not base64"),
    };

    let split = auth_token_str.split(':').collect::<Vec<&str>>();

    if split.len() > 2 {
        return bad_request_error("token should be formatted username:password");
    }

    let username = match split.get(0) {
        Some(username) => username.to_string(),
        None => return bad_request_error("token should be formatted username:password"),
    };

    let password = match split.get(1) {
        Some(username) => hash_from_password(username.to_string()),
        None => return bad_request_error("token should be formatted username:password"),
    };

    // create random session token
    let session_token = gen_session_token();

    // check to see if username and password are in our users database
    let server_data: &ServerData = req.app_data().unwrap();
    let users_db = server_data.db.collection::<User>("users");

    match users_db
        .find_one_and_update(
            doc! { "username": username.clone(), "password": password },
            doc! {"$addToSet": {"session_tokens": &session_token} },
            None,
        )
        .await
    {
        Ok(data) => match data {
            Some(_) => {
                return HttpResponse::Ok().body(format!("{{\"sessionToken\":\"{session_token}\"}}"))
            }
            None => return bad_request_error("invalid username or password"),
        },
        Err(err) => return server_error(&format!("error accessing database: {err}")),
    }
}

#[get("/api/logout")]
pub async fn logout(req: HttpRequest) -> HttpResponse {
    // validate and get session token
    let session_token = match validate_session_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid session token"),
        },
        Err(err) => return err,
    };

    // check to see if username and password are in our users database
    let server_data: &ServerData = req.app_data().unwrap();
    let users_db = server_data.db.collection::<User>("users");

    match users_db
        .find_one_and_update(
            doc! { "session_tokens": &session_token },
            doc! {"$pull": {"session_tokens": &session_token} },
            None,
        )
        .await
    {
        Ok(data) => match data {
            Some(_) => {
                return HttpResponse::Ok().finish()
            }
            None => return bad_request_error("invalid session token"),
        },
        Err(err) => return server_error(&format!("error accessing database: {err}")),
    }
}
