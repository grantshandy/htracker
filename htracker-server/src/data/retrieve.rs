use actix_web::{get, HttpRequest, HttpResponse};

use crate::{auth, bad_request_error, data::user_data_from_auth_token, ServerData};

#[get("/api/get_data")]
pub async fn get_data(req: HttpRequest) -> HttpResponse {
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

    // get user data from auth token
    let user_data = match user_data_from_auth_token(&auth_token, db).await {
        Ok(user_data) => user_data,
        Err(err) => return err,
    };

    HttpResponse::Ok().json(user_data)
}
