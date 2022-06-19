use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{bad_request_error, email, server_error, ServerData};

use super::UserInfo;

// stored in the database for users that haven't verified yet.
#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediateUserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
    pub validation_string: String,
}

#[post("/api/register")]
pub async fn register_account(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // serialize user info from request
    let user_info: UserInfo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => {
            return bad_request_error("must include username, password, and email");
        }
    };

    // make sure no fields are blank
    if user_info.username == "" || user_info.password == "" || user_info.email == "" {
        return bad_request_error("must include username, password, and email");
    }

    // validate email address
    if !mailchecker::is_valid(&user_info.email) {
        return bad_request_error("invalid email address");
    }

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;
    let users = db.collection::<UserInfo>("users");
    let intermediate_users = db.collection::<UserInfo>("intermediateUsers");

    // check against existing emails in intermediate users database
    match intermediate_users
        .find_one(doc! { "email": &user_info.email }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return bad_request_error("email already in use");
            }
        }
        Err(_) => {
            return server_error("unable to access internal database");
        }
    }

    // check against existing emails in users database
    match users
        .find_one(doc! { "email": &user_info.email }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return bad_request_error("email already in use");
            }
        }
        Err(_) => {
            return server_error("unable to access internal database");
        }
    }

    // check against existing usernames in intermediate users database
    match intermediate_users
        .find_one(doc! { "username": &user_info.username }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return bad_request_error("username already in use");
            }
        }
        Err(_) => {
            return server_error("unable to access internal database");
        }
    }

    // check against existing usernames in users database
    match users
        .find_one(doc! { "username": &user_info.username }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return bad_request_error("username already in use");
            }
        }
        Err(_) => {
            return server_error("unable to access internal database");
        }
    }

    // generate random validation string
    let validation_string = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    // create intermediate user info for database
    let intermediate_user_info = IntermediateUserInfo {
        username: user_info.username,
        password: user_info.password,
        email: user_info.email,
        validation_string,
    };

    // send verification email to user's inbox
    if let Some(err) = email::send_validation_email(&intermediate_user_info, &req).await {
        return server_error(&format!("couldn't send email: {err}"));
    };

    // insert intermediate user info into database
    if db
        .collection::<IntermediateUserInfo>("intermediateUsers")
        .insert_one(&intermediate_user_info, None)
        .await
        .is_err()
    {
        return server_error("couldn't update intermediate user database");
    };

    HttpResponse::Ok()
        .body("{\"info\":\"Validation email sent to your inbox, be sure to check your spam.\"}")
}
