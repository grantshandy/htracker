use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    auth, bad_request_error,
    data::{user_data_from_auth_token, UserData},
    server_error, ServerData,
};

// todo stored in the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InternalTodo {
    pub name: String,
    pub id: String,
}

impl InternalTodo {
    pub fn new<A: AsRef<str>>(name: A) -> Self {
        let id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        Self {
            name: name.as_ref().to_string(),
            id,
        }
    }
}

// todo that the client sends
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExternalTodo {
    pub name: String,
}

#[post("/api/add_todo")]
pub async fn add_todo(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // serialize todo data
    let todo: ExternalTodo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return bad_request_error("bad data formatting"),
    };

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
    let mut user_data = match user_data_from_auth_token(&auth_token, db).await {
        Ok(user_data) => user_data,
        Err(error) => return error,
    };

    // push new todo into user data
    user_data.todos.push(InternalTodo::new(todo.name));

    // replace db user data with new copy
    if db
        .collection::<UserData>("userData")
        .replace_one(doc! { "auth_token": &auth_token }, user_data, None)
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    HttpResponse::Ok().body("{}")
}
