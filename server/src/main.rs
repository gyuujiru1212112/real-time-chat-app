mod database;

use database::DbManager;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{launch, post, routes};

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
    let q = format!(
        "INSERT INTO user (username, email, password) VALUES (\"{}\", \"{}\", \"{}\")",
        signup_info.username, signup_info.email, signup_info.password
    );
    let success: bool = db_manager.execute(&q).await;
    if success {
        println!("User created");
        Status::Created
    } else {
        println!("Failed to create user");
        Status::InternalServerError
    }
}

#[post("/login", format = "json", data = "<user_login>")]
fn login(user_login: Json<UserLogin>) -> Status {
    println!("login username: {}", user_login.username);
    println!("login password: {}", user_login.password);
    Status::Ok
}

#[post("/logout", format = "json", data = "<user_logout>")]
fn logout(user_logout: Json<UserLogout>) -> Status {
    println!("logout username: {}", user_logout.username);
    println!("logout session_id: {}", user_logout.session_id);
    Status::Ok
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let db_url: String = String::from("mysql://chatserver:ServerPass123@localhost:3306/chatapp");
    let mut db_manager = DbManager::new(db_url);
    db_manager.connect().await;
    rocket::build()
        .manage::<DbManager>(db_manager)
        .mount("/chatapp/user/", routes![signup, login, logout])
}
