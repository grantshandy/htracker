use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::{bson::doc, Database};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{bad_request_error, email, server_error, ServerData};

use super::UserInfo;

// stored in the database for users that haven't verified yet.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
    pub validation_string: String,
}

impl NewUserInfo {
    pub fn from_user_info(user_info: &UserInfo) -> Self {
        let validation_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        Self {
            username: user_info.username.clone(),
            password: user_info.password.clone(),
            email: user_info.email.clone(),
            validation_string,
        }
    }
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

    // make sure the user isn't a duplicate
    match check_is_user_duplicate(&user_info, &db).await {
        Ok(res) => {
            if res {
                return bad_request_error("username or email already in use");
            }
        }
        Err(err) => return err,
    }

    // create new user info for database
    let new_user_info = NewUserInfo::from_user_info(&user_info);

    // send verification email to user's inbox
    if let Some(err) = email::send_validation_email(&new_user_info, &req).await {
        return server_error(&format!("couldn't send email: {err}"));
    };

    // insert new user info into database
    if db
        .collection::<NewUserInfo>("newUsers")
        .insert_one(&new_user_info, None)
        .await
        .is_err()
    {
        return server_error("couldn't update new user database");
    };

    HttpResponse::Ok()
        .body("{\"info\":\"Validation email sent to your inbox, be sure to check your spam.\"}")
}

async fn check_is_user_duplicate(
    user_info: &UserInfo,
    db: &Database,
) -> Result<bool, HttpResponse> {
    let users = db.collection::<UserInfo>("users");
    let new_users = db.collection::<UserInfo>("newUsers");

    // check against existing emails in new user database
    match new_users
        .find_one(doc! { "email": &user_info.email }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return Ok(true);
            }
        }
        Err(_) => {
            return Err(server_error("unable to access internal database"));
        }
    }

    // check against existing emails in user database
    match users
        .find_one(doc! { "email": &user_info.email }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return Ok(true);
            }
        }
        Err(_) => {
            return Err(server_error("unable to access internal database"));
        }
    }

    // check against existing usernames in new user database
    match new_users
        .find_one(doc! { "username": &user_info.username }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return Ok(true);
            }
        }
        Err(_) => {
            return Err(server_error("unable to access internal database"));
        }
    }

    // check against existing usernames in user database
    match users
        .find_one(doc! { "username": &user_info.username }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return Ok(true);
            }
        }
        Err(_) => {
            return Err(server_error("unable to access internal database"));
        }
    }

    Ok(false)
}
