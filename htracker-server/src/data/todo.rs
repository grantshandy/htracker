use actix_web::{post, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{auth, bad_request_error, data::UserData, server_error, ServerData};

use super::user_data_from_auth_token;

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
    // validate and get auth token
    let auth_token = match auth::validate_auth_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid auth token"),
        },
        Err(err) => return err,
    };

    // serialize todo data
    let todo: ExternalTodo = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return bad_request_error("bad data formatting"),
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    let todo = InternalTodo::new(todo.name);

    // replace update db
    if db
        .collection::<UserData>("userData")
        .find_one_and_update(
            doc! { "auth_token": &auth_token },
            doc! {"$addToSet" : {"todos" : {"name" : todo.name, "id" : todo.id }}},
            None,
        )
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    // return updated todos
    match user_data_from_auth_token(&auth_token, db).await {
        Ok(data) => HttpResponse::Ok().json(data.todos),
        Err(err) => err,
    }
}

#[post("/api/remove_todo")]
pub async fn remove_todo(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // validate and get auth token
    let auth_token = match auth::validate_auth_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid auth token"),
        },
        Err(err) => return err,
    };

    // serialize json data
    let json: serde_json::Value = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return bad_request_error("not formatted as json"),
    };

    let id: String = match json.get("id") {
        Some(id) => id.to_string().replace('"', ""),
        None => return bad_request_error("didn't send id"),
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // replace update db
    if db
        .collection::<UserData>("userData")
        .find_one_and_update(
            doc! { "auth_token": &auth_token },
            doc! {"$pull" : {"todos" : {"id" : id }}},
            None,
        )
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    // return updated todos
    match user_data_from_auth_token(&auth_token, db).await {
        Ok(data) => HttpResponse::Ok().json(data.todos),
        Err(err) => err,
    }
}
