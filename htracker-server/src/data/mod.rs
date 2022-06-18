use actix_web::HttpResponse;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

mod todo;

pub use todo::add_todo;

use crate::server_error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub auth_token: String,
    pub todos: Vec<Todo>,
}

impl UserData {
    pub fn new<A: AsRef<str>>(auth_token: A) -> Self {
        Self {
            auth_token: auth_token.as_ref().to_string(),
            todos: Vec::new(),
        }
    }
}

async fn user_data_from_auth_token(
    auth_token: &str,
    db: &Database,
) -> Result<UserData, HttpResponse> {
    println!("finding {auth_token}");

    match db
        .collection::<UserData>("userData")
        .find_one(doc! { "auth_token": auth_token }, None)
        .await
    {
        Ok(user_data) => match user_data {
            Some(user_data) => Ok(user_data),
            None => Err(server_error(
                "couldn't access user's userData in internal database",
            )),
        },
        Err(_) => return Err(server_error("couldn't access internal database")),
    }
}
