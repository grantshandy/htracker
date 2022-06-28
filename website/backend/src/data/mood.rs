use actix_web::{post, HttpResponse, web, HttpRequest};

use crate::{bad_request_error, auth, ServerData};

#[post("/api/log_mood")]
pub async fn log_mood(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // validate and get auth token
    let auth_token = match auth::validate_auth_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid auth token"),
        },
        Err(err) => return err,
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    HttpResponse::Ok().finish()
}
