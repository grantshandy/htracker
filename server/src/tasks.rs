use actix_web::{get, post, web, HttpRequest, HttpResponse};
use bson::doc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{self, get_user_data, User},
    bad_request_error, server_error, ServerData,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub fn gen_task_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

// task that the client sends when creating a new one
#[derive(Deserialize, Clone)]
struct AddTask {
    pub name: String,
    pub description: Option<String>,
}

#[post("/api/add_task")]
pub async fn add_task(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // validate and get session token
    let session_token = match auth::validate_session_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid session token"),
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
    if let Some(error) = db
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "session_tokens": &session_token },
            doc! {"$addToSet": {"data.tasks": bson::to_bson(&task).unwrap()}},
            None,
        )
        .await
        .err()
    {
        return server_error(format!("couldn't access internal database: {error}"));
    };

    // return updated task list
    match get_user_data(&req, session_token).await {
        Ok(data) => HttpResponse::Ok().json(data.tasks),
        Err(err) => err,
    }
}

// task that the client sends to remove_task
#[derive(Deserialize, Clone)]
struct RemoveTask {
    pub id: String,
}

#[post("/api/remove_task")]
pub async fn remove_task(bytes: web::Bytes, req: HttpRequest) -> HttpResponse {
    // validate and get session token
    let session_token = match auth::validate_session_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid session token"),
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
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "session_tokens": &session_token },
            doc! {"$pull" : {"data.tasks" : {"id" : query.id }}},
            None,
        )
        .await
        .is_err()
    {
        return server_error("couldn't access internal database");
    };

    // return updated task list
    match get_user_data(&req, session_token).await {
        Ok(data) => HttpResponse::Ok().json(data.tasks),
        Err(err) => err,
    }
}

// get all tasks
#[get("/api/get_tasks")]
pub async fn get_tasks(req: HttpRequest) -> HttpResponse {
    // validate and get session token
    let session_token = match auth::validate_session_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(auth_token) => auth_token,
            None => return bad_request_error("invalid session token"),
        },
        Err(err) => return err,
    };

    // return updated task list
    match get_user_data(&req, session_token).await {
        Ok(data) => HttpResponse::Ok().json(data.tasks),
        Err(err) => err,
    }
}
