use actix_web::{post, web, HttpRequest, HttpResponse};
use chrono::Utc;
use bson::DateTime;
use mongodb::bson::{self, doc};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{auth, bad_request_error, ServerData, server_error, data::user_data};

use super::UserData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mood {
    mood: u8,
    date: DateTime,
}

impl Mood {
    pub fn new(mood: u8) -> Self {
        Self {
            mood,
            date: DateTime::from_chrono(Utc::now()),
        }
    }
}

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

    let json: Value = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return bad_request_error("bad data formatting"),
    };

    let mood = match json.get("mood") {
        Some(mood) => match mood.as_u64() {
            Some(mood) => {
                if mood >= 6 {
                    return bad_request_error("mood must be equal or less than five");
                } else {
                    mood as u8
                }
            }
            None => return bad_request_error("mood must be a number"),
        },
        None => return bad_request_error("must include mood"),
    };

    // get handle on internal database
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    println!("logging mood at {mood}");

    let mood = Mood::new(mood);

    // update moods in database
    if db
        .collection::<UserData>("UserData")
        .find_one_and_update(
            doc! { "auth_token": &auth_token },
            doc! {"$addToSet" : {"moods" : bson::to_bson(&mood).unwrap()}},
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
