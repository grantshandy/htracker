use actix_web::{get, http::header::ContentType, post, web, HttpRequest, HttpResponse};
use mongodb::{bson::doc, Database};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{bad_request_error, email, server_error, ServerData};

use super::User;

// stored in the database for users that haven't verified yet.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub validation_string: String,
}

impl NewUser {
    pub fn from_register_request(register_request: &RegisterRequest) -> Self {
        let validation_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        Self {
            username: register_request.username.clone(),
            password: register_request.password.clone(),
            email: register_request.email.clone(),
            validation_string,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[post("/api/register")]
pub async fn register_account(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // serialize user info from request
    let register_request: RegisterRequest = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => {
            return bad_request_error("must include username, password, and email");
        }
    };

    // make sure no fields are blank
    if register_request.username == "" || register_request.password == "" || register_request.email == "" {
        return bad_request_error("must include username, password, and email");
    }

    // validate email address
    if !mailchecker::is_valid(&register_request.email) {
        return bad_request_error("invalid email address");
    }

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // make sure the user isn't a duplicate
    match check_is_user_duplicate(&register_request, &db).await {
        Ok(res) => {
            if res {
                return bad_request_error("username or email already in use");
            }
        }
        Err(err) => return err,
    }

    // create new user info for database
    let new_user = NewUser::from_register_request(&register_request);

    // send verification email to user's inbox
    if let Some(err) = email::send_validation_email(&new_user, &req).await {
        return server_error(&format!("couldn't send email: {err}"));
    };

    // insert new user info into database
    if db
        .collection::<NewUser>("newUsers")
        .insert_one(&new_user, None)
        .await
        .is_err()
    {
        return server_error("couldn't update new user database");
    };

    HttpResponse::Ok()
        .body(r##"{"info":"Validation email sent to your inbox, be sure to check your spam."}"##)
}

/// validate a user's account in the db
#[get("/validate/{validation_string}")]
pub async fn validate_account(
    validation_string: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let validation_string = validation_string.to_string();

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // get handle to db collections
    let users = db.collection::<User>("users");
    let new_users = db.collection::<NewUser>("newUsers");

    // retrieve new user info from validation string
    let new_user: NewUser = match new_users
        .find_one(doc! { "validation_string": &validation_string }, None)
        .await
    {
        Ok(info) => match info {
            Some(info) => info,
            None => return server_error("invalid validation string"),
        },
        Err(err) => return server_error(&format!("Couldn't search validation string: {err}")),
    };

    // remove from new database
    if let Err(err) = new_users
        .delete_one(doc! { "validation_string": &validation_string }, None)
        .await
    {
        return server_error(&format!("Couldn't search validation string: {err}"));
    };

    // create normal user info from new user info
    let user = User::from_new(&new_user);

    // insert user info into user database.
    if let Err(err) = users.insert_one(&user, None).await {
        return server_error(&format!("Couldn't update user info: {err}"));
    };

    HttpResponse::Ok().content_type(ContentType::html()).body(
        r##"<!DOCTYPE html>
        <html>
        <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
        </head>
        <body>
          <h1>success, redirecting...</h1>
          <script>window.location.href = '/login'</script>
        </body>
        </html>"##,
    )
}

async fn check_is_user_duplicate(user: &RegisterRequest, db: &Database) -> Result<bool, HttpResponse> {
    let users = db.collection::<User>("users");
    let new_users = db.collection::<NewUser>("newUsers");

    // check against existing emails in new user database
    match new_users
        .find_one(doc! { "email": &user.email }, None)
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
    match users.find_one(doc! { "email": &user.email }, None).await {
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
        .find_one(doc! { "username": &user.username }, None)
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
        .find_one(doc! { "username": &user.username }, None)
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
