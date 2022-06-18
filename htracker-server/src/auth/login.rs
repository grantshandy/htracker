use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{server_error, ServerData};

use super::UserInfo;

#[derive(Serialize, Deserialize, Debug)]
struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[post("/api/login")]
pub async fn login_auth(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // parse into login info
    let login_info: LoginInfo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => {
            return HttpResponse::BadRequest()
                .body("{\"error\":\"Must Include Username and Password\"}")
        }
    };

    // check to see if username and password are empty
    if login_info.username == "" || login_info.password == "" {
        return HttpResponse::BadRequest()
            .body("{\"error\":\"Must Include Username and Password\"}");
    }

    // check against internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let users = server_data.db.collection::<UserInfo>("users");
    let valid = match users
        .find_one(
            doc! { "username": login_info.username, "password": login_info.password },
            None,
        )
        .await
    {
        Ok(user) => user.is_some(),
        Err(_) => return server_error("Couldn't access internal database"),
    };

    // another use of manual json formatting
    HttpResponse::Ok().body(format!("{{\"valid\":{valid}}}"))
}
