mod database;

use database::DbManager;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{launch, post, routes};
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
struct UserLogout {
    username: String,
    session_id: String,
}

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
                    .set_user_session_id(&user_login.username, &session_id)
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

#[post("/logout", format = "json", data = "<user_logout>")]
async fn logout(user_logout: Json<UserLogout>, db_manager: &rocket::State<DbManager>) -> Status {
    match db_manager.get_user(&user_logout.username).await {
        Some(user) => match user.session_id {
            Some(session_id) => {
                if session_id != user_logout.session_id {
                    Status::Unauthorized
                } else {
                    let success = db_manager
                        .set_user_session_id(&user_logout.username, &String::new())
                        .await;
                    if success {
                        Status::Ok
                    } else {
                        Status::InternalServerError
                    }
                }
            }
            None => Status::Unauthorized,
        },
        None => Status::Unauthorized,
    }
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let db_url: String = String::from("mysql://chatserver:ServerPass123@localhost:3306/chatapp");
    rocket::build()
        .manage::<DbManager>(DbManager::new(db_url).await.unwrap())
        .mount("/chatapp/user/", routes![signup, login, logout])
}
