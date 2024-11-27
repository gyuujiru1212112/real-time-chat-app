use crate::database::DbManager;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserReqInfo {
    pub username: String,
    pub session_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserReqInfo {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_id = request.headers().get_one("session_id").unwrap_or("");
        let username = request.headers().get_one("username").unwrap_or("");

        Outcome::Success(UserReqInfo {
            username: String::from(username),
            session_id: String::from(session_id),
        })
    }
}

pub async fn is_session_id_valid(
    username: &String,
    given_session_id: &String,
    db_manager: &DbManager,
) -> bool {
    let mut is_valid: bool = false;
    match db_manager.get_user(username).await {
        Some(user) => match user.session_id {
            Some(session_id) => {
                if session_id == *given_session_id {
                    println!("Valid username and session_id pair.");
                    is_valid = true;
                } else {
                    println!(
                        "Given session_id does not match with expected value for user \'{}\'.",
                        username
                    );
                }
            }
            None => println!("No current session_id for user \'{}\'.", username),
        },
        None => println!("Invalid username. Could not find user \'{}\'.", username),
    }
    is_valid
}
