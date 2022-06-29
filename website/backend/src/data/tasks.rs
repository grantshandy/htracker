use actix_web::{get, post, web, HttpRequest, HttpResponse};
use mongodb::bson::{self, doc};
use serde::{Deserialize, Serialize};

use crate::{auth, bad_request_error, data::UserData, server_error, ServerData};

use super::{gen_task_id, user_data, Task};

// task that the client sends when creating a new one
#[derive(Serialize, Deserialize, Clone)]
struct AddTask {
    pub name: String,
    pub description: Option<String>,
}

#[post("/api/add_task")]
pub async fn add_task(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // validate and get auth token
    let auth_token = match auth::validate_auth_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid auth token"),
        },
        Err(err) => return err,
    };

    // serialize todo data
    let task: AddTask = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return bad_request_error("bad data formatting"),
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // create new internal todo with id
    let task = Task {
        name: task.name,
        description: task.description,
        id: gen_task_id(),
    };

    // replace update db
    if db
        .collection::<UserData>("userData")
        .find_one_and_update(
            doc! { "auth_token": &auth_token },
            doc! {"$addToSet" : {"tasks" : bson::to_bson(&task).unwrap()}},
            None,
        )
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    // return updated task list
    match user_data(&auth_token, db).await {
        Ok(data) => HttpResponse::Ok().json(data.tasks),
        Err(err) => err,
    }
}

// task that the client sends to remove_task
#[derive(Serialize, Deserialize, Clone)]
struct RemoveTask {
    pub id: String,
}

#[post("/api/remove_task")]
pub async fn remove_task(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // validate and get auth token
    let auth_token = match auth::validate_auth_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid auth token"),
        },
        Err(err) => return err,
    };

    // get query from request
    let query: RemoveTask = match serde_json::from_slice(&bytes) {
        Ok(query) => query,
        Err(_) => return bad_request_error("not formatted as json (must include id!)"),
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // remove todo from task list in db
    if db
        .collection::<UserData>("userData")
        .find_one_and_update(
            doc! { "auth_token": &auth_token },
            doc! {"$pull" : {"tasks" : {"id" : query.id }}},
            None,
        )
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    // return updated tasks
    match user_data(&auth_token, db).await {
        Ok(data) => HttpResponse::Ok().json(data.tasks),
        Err(err) => err,
    }
}

// get all tasks
#[get("/api/get_tasks")]
pub async fn get_tasks(req: HttpRequest) -> HttpResponse {
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

    // return user data
    match user_data(&auth_token, db).await {
        Ok(user_data) => HttpResponse::Ok().json(user_data.tasks),
        Err(err) => err,
    }
}
