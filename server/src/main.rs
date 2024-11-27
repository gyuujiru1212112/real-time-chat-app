mod database;

use database::DbManager;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct SignupInfo {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct UserLogin {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct UserReqInfo {
    username: String,
    session_id: String,
}

// #[derive(Deserialize, Serialize)]
// struct UserStatus {
//     username: String,
//     session_id: String,
// }

#[post("/signup", format = "json", data = "<signup_info>")]
async fn signup<'a>(
    signup_info: Json<SignupInfo>,
    db_manager: &rocket::State<DbManager>,
) -> Status {
    let success: bool = db_manager
        .insert_user(
            &signup_info.username,
            &signup_info.email,
            &signup_info.password,
        )
        .await;
    if success {
        println!("User created");
        Status::Created
    } else {
        println!("Failed to create user {}", signup_info.username);
        Status::InternalServerError
    }
}

#[post("/login", format = "json", data = "<user_login>")]
async fn login(
    user_login: Json<UserLogin>,
    db_manager: &rocket::State<DbManager>,
) -> (Status, String) {
    match db_manager.get_user(&user_login.username).await {
        Some(user) => {
            if user.password != user_login.password {
                (
                    Status::Unauthorized,
                    String::from("{\"message\": \"Login Failed\"}"),
                )
            } else {
                let session_id: String = Uuid::new_v4().to_string();
                let success = db_manager
                    .set_user_session_id(&user_login.username, Some(&session_id))
                    .await;
                if success {
                    let response_body = format!(
                        "{{\"message\": \"Success\", \"session_id\": \"{}\"}}",
                        session_id
                    );
                    (Status::Ok, response_body)
                } else {
                    (
                        Status::InternalServerError,
                        String::from("{\"message\": \"Login Failed\"}"),
                    )
                }
            }
        }
        None => (
            Status::Unauthorized,
            String::from("{\"message\": \"Login Failed\"}"),
        ),
    }
}

#[post("/logout", format = "json", data = "<user>")]
async fn logout(user: Json<UserReqInfo>, db_manager: &rocket::State<DbManager>) -> Status {
    let valid_session: bool =
        is_session_id_valid(&user.username, &user.session_id, db_manager.inner()).await;

    if valid_session {
        let success = db_manager.set_user_session_id(&user.username, None).await;
        if success {
            Status::Ok
        } else {
            Status::InternalServerError
        }
    } else {
        Status::Unauthorized
    }
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

#[get("/status?<username..>")]
async fn active_users(
    username: Option<String>,
    user_info: UserReqInfo,
    db_manager: &rocket::State<DbManager>,
) -> (Status, String) {
    let valid_session: bool = is_session_id_valid(
        &user_info.username,
        &user_info.session_id,
        db_manager.inner(),
    )
    .await;

    if valid_session {
        match username {
            Some(name) => match db_manager.get_user(&name).await {
                Some(user) => {
                    let status = match user.session_id {
                        Some(_) => "ACTIVE",
                        None => "INACTIVE",
                    };
                    (
                        Status::Ok,
                        format!(
                            "[{{\"username\": \"{}\", \"status\": \"{}\"}}]",
                            user.username, status
                        ),
                    )
                }
                None => (
                    Status::NotFound,
                    format!(
                        "[{{\"username\": \"{}\", \"status\": \"NOT_FOUND\"}}]",
                        name
                    ),
                ),
            },
            None => match db_manager.get_active_users().await {
                Some(users) => {
                    let mut ret_message = String::new();
                    ret_message += "[";
                    for user in users.iter() {
                        ret_message += &format!(
                            "{{\"username\": \"{}\", \"status\": \"ACTIVE\"}},",
                            user.username
                        );
                    }
                    ret_message.pop(); // Remove last character "," from the string.
                    ret_message += "]";

                    (Status::Ok, ret_message)
                }
                None => (Status::Ok, String::from("[]")),
            },
        }
    } else {
        (Status::Unauthorized, String::from(""))
    }
}

async fn is_session_id_valid(
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

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let db_url: String = String::from("mysql://chatserver:ServerPass123@localhost:3306/chatapp");
    rocket::build()
        .manage::<DbManager>(DbManager::new(db_url).await.unwrap())
        .mount(
            "/chatapp/user/",
            routes![signup, login, logout, active_users],
        )
}
