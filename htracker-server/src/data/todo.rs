use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;

use crate::{
    auth, bad_request_error,
    data::{user_data_from_auth_token, Todo, UserData},
    server_error, ServerData,
};

#[post("/api/add_todo")]
pub async fn add_todo(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // serialize todo data
    let todo: Todo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return bad_request_error("bad data formatting"),
    };

    // validate and get auth token
    let auth_token = match auth::validate_auth_token(&req).await {
        Ok(auth_token) => auth_token,
        Err(err) => return err,
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // get user data from auth token
    let mut user_data = match user_data_from_auth_token(&auth_token, db).await {
        Ok(user_data) => user_data,
        Err(error) => return error,
    };

    // push new todo into user data
    user_data.todos.push(todo);

    // replace db user data with new copy
    if db
        .collection::<UserData>("userData")
        .replace_one(doc! { "auth_token": &auth_token }, user_data, None)
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    HttpResponse::Ok().finish()
}
