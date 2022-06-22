use actix_web::{get, HttpResponse, HttpRequest};
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{bad_request_error, auth};

lazy_static! {
    // parse csv here from scratch :)
    pub static ref QUOTES: Vec<Quote> = {
        let text = include_str!("../res/quotes");

        let mut res = Vec::new();

        for line in text.lines() {
            let split = line.split('|').collect::<Vec<&str>>();

            let author = split[0].to_string();
            let text = split[1].to_string();

            res.push(Quote {
                text,
                author,
            });
        }

        res
    };
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

#[get("/api/quote")]
pub async fn quote(req: HttpRequest) -> HttpResponse {
    // validate and get auth token
    match auth::validate_auth_token(&req).await {
        Ok(auth_token) => match auth_token {
            Some(_) => (),
            None => return bad_request_error("invalid auth token"),
        },
        Err(err) => return err,
    };

    // choose random quote from quotes list
    let mut rng = rand::thread_rng();

    let quote = QUOTES.choose(&mut rng).unwrap();

    HttpResponse::Ok().json(quote)
}
