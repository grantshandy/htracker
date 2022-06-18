use mailjet_rs::{common::Recipient, v3::Message, Client, SendAPIVersion};

use crate::{auth::IntermediateUserInfo, BASE_URL};

const MAILJET_PUBLIC_KEY: &'static str = include_str!("../MAILJET_PUBLIC_KEY");
const MAILJET_SECRET_KEY: &'static str = include_str!("../MAILJET_SECRET_KEY");

const VALIDATION_EMAIL: &'static str = "----------------------------------
htracker validation
----------------------------------

Hello, NAME, click the link below to validate your account.
";

const SENDER_EMAIL: &'static str = "validation@htracker.xyz";
const SENDER_NAME: &'static str = "Htracker Account Validation";

pub async fn send_validation_email(user_info: &IntermediateUserInfo) -> Option<String> {
    let client = Client::new(SendAPIVersion::V3, MAILJET_PUBLIC_KEY, MAILJET_SECRET_KEY);

    let mut email_contents = String::from(VALIDATION_EMAIL).replace("NAME", &user_info.username);
    email_contents.push_str(&format!(
        "{}/api/verify/{}",
        BASE_URL, user_info.validation_string
    ));

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        SENDER_EMAIL,
        SENDER_NAME,
        Some(format!(
            "Hello, {}, Validate Your Htracker Account",
            &user_info.username
        )),
        Some(email_contents),
    );

    message.push_recipient(Recipient::new(&user_info.email));

    match client.send(message).await {
        Ok(_) => None,
        Err(err) => Some(err.message),
    }
}
