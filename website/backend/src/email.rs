use actix_web::HttpRequest;
use mailjet_rs::{common::Recipient, v3::Message, Client, SendAPIVersion};

use crate::{auth::NewUser, ServerData};

const MAILJET_PUBLIC_KEY: &'static str = include_str!("../res/MAILJET_PUBLIC_KEY");
const MAILJET_SECRET_KEY: &'static str = include_str!("../res/MAILJET_SECRET_KEY");

const VALIDATION_EMAIL: &'static str = "----------------------------------
htracker validation
----------------------------------

Hello, NAME, click the link below to validate your account.
";

const SENDER_EMAIL: &'static str = "validation@htracker.xyz";
const SENDER_NAME: &'static str = "Htracker Account Validation";

pub async fn send_validation_email(new_user: &NewUser, req: &HttpRequest) -> Option<String> {
    let server_data: &ServerData = req.app_data().unwrap();
    let base_url = &server_data.args.base_url;

    let client = Client::new(SendAPIVersion::V3, MAILJET_PUBLIC_KEY, MAILJET_SECRET_KEY);

    let mut email_contents = String::from(VALIDATION_EMAIL).replace("NAME", &new_user.username);
    email_contents.push_str(&format!(
        "{}/validate/{}",
        base_url, new_user.validation_string
    ));

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        SENDER_EMAIL,
        SENDER_NAME,
        Some(format!(
            "{}, Validate Your Htracker Account",
            &new_user.username
        )),
        Some(email_contents),
    );

    message.push_recipient(Recipient::new(&new_user.email));

    match client.send(message).await {
        Ok(_) => None,
        Err(err) => Some(err.message),
    }
}
