use crate::database::DbManager;
use crate::endpoints::common::{is_session_id_valid, UserReqInfo};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct SignupInfo {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLogin {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserStatus {
    username: String,
    status: String,
}

#[post("/signup", format = "json", data = "<signup_info>")]
pub async fn signup<'a>(
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
pub async fn login(
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
pub async fn logout(user: Json<UserReqInfo>, db_manager: &rocket::State<DbManager>) -> Status {
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

#[get("/status?<username..>")]
pub async fn user_status(
    username: String,
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
        match db_manager.get_user(&username).await {
            Some(user) => match user.session_id {
                Some(_) => (Status::Ok, String::from("ACTIVE")),
                None => (Status::Ok, String::from("INACTIVE")),
            },
            None => (Status::NotFound, String::from("NOT_FOUND")),
        }
    } else {
        (Status::Unauthorized, String::from(""))
    }
}

#[get("/allusers")]
pub async fn active_users(
    user_info: UserReqInfo,
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<Vec<UserStatus>>) {
    let valid_session: bool = is_session_id_valid(
        &user_info.username,
        &user_info.session_id,
        db_manager.inner(),
    )
    .await;

    if valid_session {
        match db_manager.get_all_users().await {
            Some(users) => {
                let res: Vec<UserStatus> = users
                    .iter()
                    .map(|user| {
                        let status = if user.session_id.is_some() {
                            "ACTIVE"
                        } else {
                            "INACTIVE"
                        };

                        UserStatus {
                            username: user.username.clone(),
                            status: status.to_string(),
                        }
                    })
                    .collect();
                (Status::Ok, Json(res))
            }
            None => (Status::Ok, Json(vec![])),
        }
    } else {
        (Status::Unauthorized, Json(vec![]))
    }
}
