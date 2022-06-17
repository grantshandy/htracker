use actix_web::{get, post, web, HttpRequest, HttpResponse};
use log::error;
use mongodb::bson::doc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{email, ServerData};

pub fn gen_error_page(message: &str) -> String {
    String::from(include_str!("../public/error_page.html")).replace("MESSAGE", message)
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediateUserInfo {
    pub username: String,
    pub password: String,
    pub email: String,
    pub validation_string: String,
}

#[post("/api/login")]
pub async fn login(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    let login_info: LoginInfo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => {
            return HttpResponse::BadRequest()
                .body("{\"error\":\"Must Include Username and Password\"}")
        }
    };

    if login_info.username == "" || login_info.password == "" {
        return HttpResponse::BadRequest()
            .body("{\"error\":\"Must Include Username and Password\"}");
    }

    let server_data: &ServerData = req.app_data().unwrap();
    let htracker_db = &server_data.htracker_db;

    let users_collection = htracker_db.collection::<UserInfo>("users");
    let login_res = users_collection
        .find_one(
            doc! { "username": login_info.username, "password": login_info.password },
            None,
        )
        .await
        .unwrap();

    let valid = login_res.is_some();

    // another use of manual json formatting
    HttpResponse::Ok().body(format!("{{\"valid\":{valid}}}"))
}

#[post("/api/register")]
pub async fn register(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    let user_info: UserInfo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => {
            return HttpResponse::BadRequest()
                .body("{\"error\":\"Must Include Username, Password and Email\"}")
        }
    };

    if user_info.username == "" || user_info.password == "" || user_info.email == "" {
        return HttpResponse::BadRequest()
            .body("{\"error\":\"Must Include Username, Password and Email\"}");
    }

    if !mailchecker::is_valid(&user_info.email) {
        return HttpResponse::BadRequest().body("{\"error\":\"Invalid Email Address\"}");
    }

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let htracker_db = &server_data.htracker_db;
    let unvalidated_users_collection =
        htracker_db.collection::<IntermediateUserInfo>("intermediateUsers");
    let users_collection = htracker_db.collection::<UserInfo>("users");

    // check against existing accounts
    match unvalidated_users_collection
        .find_one(doc! { "email": &user_info.email }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return HttpResponse::BadRequest()
                    .body(format!("{{\"error\":\"Email Already Exists\"}}"));
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{err}\"}}"))
        }
    }

    match users_collection
        .find_one(doc! { "email": &user_info.email }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return HttpResponse::BadRequest()
                    .body(format!("{{\"error\":\"Email Already Exists\"}}"));
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{err}\"}}"))
        }
    }

    match unvalidated_users_collection
        .find_one(doc! { "username": &user_info.username }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return HttpResponse::BadRequest()
                    .body(format!("{{\"error\":\"Username Already Exists\"}}"));
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{err}\"}}"))
        }
    }

    match users_collection
        .find_one(doc! { "username": &user_info.username }, None)
        .await
    {
        Ok(res) => {
            if res.is_some() {
                return HttpResponse::BadRequest()
                    .body(format!("{{\"error\":\"Username Already Exists\"}}"));
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{err}\"}}"))
        }
    }

    // generate random validation string
    let validation_string = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let intermediate_user_info = IntermediateUserInfo {
        username: user_info.username,
        password: user_info.password,
        email: user_info.email,
        validation_string,
    };

    match unvalidated_users_collection
        .insert_one(&intermediate_user_info, None)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{err}\"}}"))
        }
    };

    if let Some(err) = email::send_validation_email(&intermediate_user_info).await {
        error!("Couldn't send email {err}");
        return HttpResponse::InternalServerError()
            .body(format!("{{\"error\":\"Couldn't send email {err}\"}}"));
    };

    HttpResponse::Ok()
        .body("{\"info\":\"Validation email sent to your inbox, be sure to check your spam.\"}")
}

#[get("/api/verify/{validation_string}")]
pub async fn verify(validation_string: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let validation_string = validation_string.to_string();

    println!("verify");

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let htracker_db = &server_data.htracker_db;
    let unvalidated_users_collection =
        htracker_db.collection::<IntermediateUserInfo>("intermediateUsers");
    let users_collection = htracker_db.collection::<UserInfo>("users");

    // retrieve user info from intermediate collection
    let intermediate_user_info: IntermediateUserInfo = match unvalidated_users_collection
        .find_one(doc! { "validation_string": &validation_string }, None)
        .await
    {
        Ok(info) => match info {
            Some(info) => info,
            None => {
                return HttpResponse::InternalServerError()
                    .body(gen_error_page("Invalid validation string"));
            }
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(gen_error_page(&format!(
                "Couldn't search validation string: {err}"
            )));
        }
    };

    // remove from intermediate collection
    if let Err(err) = unvalidated_users_collection.delete_one(doc! { "validation_string": &validation_string, "username": &intermediate_user_info.username, "email": &intermediate_user_info.email, "password": &intermediate_user_info.password }, None).await {
        return HttpResponse::InternalServerError().body(format!("<p>Couldn't search validation string: {err}</p>"))
    };

    let user_info = UserInfo {
        username: intermediate_user_info.username,
        password: intermediate_user_info.password,
        email: intermediate_user_info.email,
    };

    // insert intermedate user data into official user database.
    if let Err(err) = users_collection.insert_one(&user_info, None).await {
        return HttpResponse::InternalServerError().body(gen_error_page(&format!(
            "<p>Couldn't update user data {err}</p>"
        )));
    };

    HttpResponse::Ok()
        .append_header(("Location", "/"))
        .body("<p>it worked!<p>")
}
