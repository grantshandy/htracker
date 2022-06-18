use actix_web::HttpResponse;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

mod retrieve;
mod todo;

pub use retrieve::get_data;
pub use todo::add_todo;

use crate::server_error;

use self::todo::InternalTodo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub auth_token: String,
    pub todos: Vec<InternalTodo>,
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
