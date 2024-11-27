mod database;
mod endpoints;

use database::DbManager;
use endpoints::user::{active_users, login, logout, signup};
use rocket::{launch, routes};

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
