use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use mongodb::bson::doc;

use crate::{data::UserData, server_error, ServerData};

use super::{gen_auth_key, NewUserInfo, UserInfo};

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

/// validate a user's account in the db
#[get("/validate/{validation_string}")]
pub async fn validate_account(
    validation_string: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let validation_string = validation_string.to_string();

    // get handle to internal databse
    let server_data: &ServerData = req.app_data().unwrap();
    let db = &server_data.db;

    // get handle to db collections
    let users = db.collection::<UserInfo>("users");
    let new_users = db.collection::<NewUserInfo>("newUsers");
    let user_data = db.collection::<UserData>("userData");

    // retrieve new user info from validation string
    let new_user_info: NewUserInfo = match new_users
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

    // remove from new database
    if let Err(err) = new_users
        .delete_one(doc! { "validation_string": &validation_string }, None)
        .await
    {
        return server_error(&format!("Couldn't search validation string: {err}"));
    };

    // create normal user info from new user info
    let user_info = UserInfo::from_new(&new_user_info);

    // insert user info into user database.
    if let Err(err) = users.insert_one(&user_info, None).await {
        return server_error(&format!("Couldn't update user info: {err}"));
    };

    // insert boilerplate user data into database
    if let Err(err) = user_data
        .insert_one(
            UserData::new(gen_auth_key(&user_info.username, &user_info.password)),
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
