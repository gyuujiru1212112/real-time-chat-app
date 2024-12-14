mod database;
mod endpoints;

use database::DbManager;
use endpoints::{
    chat::{
        create_chat_room, create_private_chat, get_all_chat_rooms, get_all_recipients,
        join_chat_room, resume_private_chat,
    },
    user::{all_users, login, logout, signup, user_status},
};
use rocket::{launch, routes};

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let db_url: String = String::from("mysql://chatserver:ServerPass123@localhost:3306/chatapp");
    rocket::build()
        .manage::<DbManager>(DbManager::new(db_url).await.unwrap())
        .mount(
            "/chatapp/user/",
            routes![signup, login, logout, user_status, all_users],
        )
        .mount(
            "/chatapp/chat/",
            routes![
                create_private_chat,
                create_chat_room,
                get_all_chat_rooms,
                get_all_recipients,
                resume_private_chat,
                join_chat_room
            ],
        )
}
