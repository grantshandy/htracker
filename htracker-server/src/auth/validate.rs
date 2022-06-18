use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;

use crate::{data::UserData, server_error, ServerData};

use super::{gen_auth_key, IntermediateUserInfo, UserInfo};

#[get("/validate/{validation_string}")]
pub async fn validate_account(
    validation_string: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let validation_string = validation_string.to_string();

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;
    let users = db.collection::<UserInfo>("users");
    let intermediate_users = db.collection::<IntermediateUserInfo>("intermediateUsers");
    let user_data = db.collection::<UserData>("userData");

    // retrieve user info from intermediate collection
    let intermediate_user_info: IntermediateUserInfo = match intermediate_users
        .find_one(doc! { "validation_string": &validation_string }, None)
        .await
    {
        Ok(info) => match info {
            Some(info) => info,
            None => {
                return server_error("invalid validation string");
            }
        },
        Err(err) => {
            return server_error(&format!("Couldn't search validation string: {err}"));
        }
    };

    // remove from intermediate database
    if let Err(err) = intermediate_users
        .delete_one(doc! { "validation_string": &validation_string }, None)
        .await
    {
        return server_error(&format!("Couldn't search validation string: {err}"));
    };

    // create user info from intermediate
    let user_info = UserInfo {
        username: intermediate_user_info.username,
        password: intermediate_user_info.password,
        email: intermediate_user_info.email,
    };

    // insert user info into user database.
    if let Err(err) = users.insert_one(&user_info, None).await {
        return server_error(&format!("Couldn't update user info: {err}"));
    };

    // insert user data into user database
    if let Err(err) = user_data
        .insert_one(
            UserData::new(gen_auth_key(user_info.username, user_info.password)),
            None,
        )
        .await
    {
        return server_error(&format!("Couldn't update user data: {err}"));
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(REDIRECT_PAGE)
}

const REDIRECT_PAGE: &'static str = r##"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body>
  <h1>success, redirecting...</h1>
  <script>window.location.href = '/login'</script>
</body>
</html>"##;
