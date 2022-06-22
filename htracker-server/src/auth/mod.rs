mod register;
mod validate;

use actix_web::{get, HttpRequest, HttpResponse};
use mongodb::bson::doc;
pub use register::{register_account, NewUserInfo};
use serde::{Deserialize, Serialize};
pub use validate::validate_account;

use crate::{bad_request_error, server_error, ServerData};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl UserInfo {
    pub fn from_new(new: &NewUserInfo) -> Self {
        Self {
            username: new.username.clone(),
            password: new.password.clone(),
            email: new.email.clone(),
        }
    }
}

pub async fn validate_auth_token(req: &HttpRequest) -> Result<Option<String>, HttpResponse> {
    // get token bytes from header
    let auth_token = match req.headers().get("X-AuthToken") {
        Some(token) => token,
        None => return Err(bad_request_error("must include token")),
    };

    // base64 token in utf8
    let auth_token = match String::from_utf8(auth_token.as_bytes().to_vec()) {
        Ok(data) => data,
        Err(_) => return Err(bad_request_error("couldn't turn encoded token into utf8")),
    };

    // username and password from auth token
    let (username, password) = username_password_from_auth_token(&auth_token)?;

    // check to see if username and password are in our users database
    let server_data: &ServerData = req.app_data().unwrap();
    match &server_data
        .db
        .collection::<UserInfo>("users")
        .find_one(doc! { "username": username, "password": password }, None)
        .await
    {
        Ok(data) => {
            if data.is_none() {
                return Ok(None);
            }
        }
        Err(err) => return Err(server_error(&format!("error accessing database: {err}"))),
    };

    Ok(Some(auth_token))
}

/// get username and password from utf8 base64 auth_token.
pub fn username_password_from_auth_token<A: AsRef<str>>(
    auth_token: A,
) -> Result<(String, String), HttpResponse> {
    let auth_token = match base64::decode(auth_token.as_ref()) {
        Ok(auth_token) => match String::from_utf8(auth_token) {
            Ok(auth_token) => auth_token,
            Err(_) => return Err(bad_request_error("token not utf8")),
        },
        Err(_) => return Err(bad_request_error("token not base64")),
    };

    let split = auth_token.split(':').collect::<Vec<&str>>();

    if split.len() > 2 {
        return Err(bad_request_error(
            "token should be formatted username:password",
        ));
    }

    let username = match split.get(0) {
        Some(username) => username.to_string(),
        None => {
            return Err(bad_request_error(
                "token should be formatted username:password",
            ))
        }
    };

    let password = match split.get(1) {
        Some(username) => username.to_string(),
        None => {
            return Err(bad_request_error(
                "token should be formatted username:password",
            ))
        }
    };

    Ok((username, password))
}

pub fn gen_auth_key<A: AsRef<str>>(username: A, password: A) -> String {
    base64::encode(format!("{}:{}", username.as_ref(), password.as_ref()))
}

#[get("/api/auth")]
pub async fn auth(req: HttpRequest) -> HttpResponse {
    let valid = match validate_auth_token(&req).await {
        Ok(res) => match res {
            Some(_) => true,
            None => false,
        },
        Err(err) => return err,
    };

    // another use of manual json formatting
    HttpResponse::Ok().body(format!("{{\"valid\":\"{valid}\"}}"))
}
