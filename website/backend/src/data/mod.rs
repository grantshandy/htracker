use actix_web::HttpResponse;
use mongodb::{bson::doc, Database};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

mod tasks;

pub use tasks::{add_task, get_tasks, remove_task};

use crate::server_error;


/// get user data from auth token and db
async fn user_data(auth_token: &str, db: &Database) -> Result<UserData, HttpResponse> {
    match db
        .collection::<UserData>("userData")
        .find_one(doc! { "auth_token": auth_token }, None)
        .await
    {
        Ok(user_data) => match user_data {
            Some(user_data) => Ok(user_data),
            None => Err(server_error("couldn't access userData")),
        },
        Err(_) => return Err(server_error("couldn't access database")),
    }
}

/// struct for the user's entire dataset
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub auth_token: String,
    pub tasks: Vec<Task>,
}

impl UserData {
    /// initialize user's userdata
    pub fn new<A: AsRef<str>>(auth_token: A) -> Self {
        Self {
            auth_token: auth_token.as_ref().to_string(),
            tasks: Vec::new(),
        }
    }
}

/// task stored in the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    // name that the user sees for the task
    pub name: String,
    pub description: Option<String>,
    // 10 char alphanumeric string used for identification.
    // used so duplicate tasks can be created.
    pub id: String,
}

pub fn gen_task_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}
