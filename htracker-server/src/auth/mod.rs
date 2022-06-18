mod login;
mod register;
mod verify;

use actix_web::{HttpRequest, HttpResponse};
pub use login::login_auth;
use mongodb::bson::doc;
pub use register::{register_account, IntermediateUserInfo};
use serde::{Deserialize, Serialize};
pub use verify::verify_account;

use crate::{bad_request_error, server_error, ServerData};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub async fn validate_auth_token(req: &HttpRequest) -> Result<String, HttpResponse> {
    // get token bytes from header
    let auth_token = match req.headers().get("X-AuthToken") {
        Some(token) => token,
        None => return Err(bad_request_error("must include token")),
    };

    // turn token bytes into utf8 text
    let auth_token_decoded = match base64::decode(auth_token.clone()) {
        Ok(data) => match String::from_utf8(data) {
            Ok(data) => data,
            Err(_) => return Err(bad_request_error("token not formatted in utf8")),
        },
        Err(_) => return Err(bad_request_error("token not formatted in base64")),
    };

    let auth_token_text = match String::from_utf8(auth_token.as_bytes().to_vec()) {
        Ok(data) => data,
        Err(_) => return Err(bad_request_error("couldn't turn encoded token into utf8")),
    };

    // split token at ':'
    let auth_token_decoded_split = &auth_token_decoded.split(':').collect::<Vec<&str>>();

    // get username before ':'
    let username = match auth_token_decoded_split.get(0) {
        Some(username) => username,
        None => {
            return Err(bad_request_error(
                "token must be formatted like username:password",
            ))
        }
    };

    // get password after ':'
    let password = match auth_token_decoded_split.get(1) {
        Some(password) => password,
        None => {
            return Err(bad_request_error(
                "token must be formatted like username:password",
            ))
        }
    };

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
                return Err(bad_request_error("invalid username or password"));
            }
        }
        Err(err) => return Err(server_error(&format!("error accessing database: {err}"))),
    };

    Ok(auth_token_text)
}

pub fn gen_auth_key<A: AsRef<str>>(username: A, password: A) -> String {
    base64::encode(format!("{}:{}", username.as_ref(), password.as_ref()))
}
