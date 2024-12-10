use rocket::http::Status;
use rocket::{get, post};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::database::DbManager;

#[derive(Deserialize, Serialize)]
pub struct ChatRoom {
    id: String,
    name: String
}

#[derive(Deserialize, Serialize)]
pub struct ChatRoomInfo {
    name: String,
    users: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PrivateChatInfo {
    user1: String,
    user2: String
}

#[post("/private-chat", format = "json", data = "<private_chat_info>")]
pub async fn create_private_chat<'a>(
    private_chat_info: Json<PrivateChatInfo>,
    db_manager: &rocket::State<DbManager>,) 
    -> Status
{
    let success: bool = db_manager
        .insert_private_chat(
            &private_chat_info.user1,
            &private_chat_info.user2
        ).await;

        if success {
            println!("Private chat created between users '{}' and '{}'", private_chat_info.user1, private_chat_info.user2);
            Status::Created
        } else {
            println!("Failed to create a private chat between users '{}' and '{}'", private_chat_info.user1, private_chat_info.user2);
            Status::InternalServerError
        }

}

#[post("/chat-room", format = "json", data = "<chat_room_info>")]
pub async fn create_chat_room<'a>(
    chat_room_info: Json<ChatRoomInfo>,
    db_manager: &rocket::State<DbManager>,
) -> Status {
    let success: bool = db_manager
        .insert_chat_room(
            &chat_room_info.name,
            &chat_room_info.users
        )
        .await;
    if success {
        println!("Chat room '{}' created", chat_room_info.name);
        Status::Created
    } else {
        println!("Failed to create chat room '{}'.", chat_room_info.name);
        Status::InternalServerError
    }
}

#[get("/all-chatroom")]
pub async fn get_all_chat_rooms(
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<Vec<ChatRoom>>) {
    match db_manager.get_all_chat_rooms().await {
        Some(rooms) => {
            let res: Vec<ChatRoom> = rooms.iter().map(|room| {
                ChatRoom {
                    id: room.0.clone(),
                    name: room.1.clone(), // Assuming room.1 is a String
                }
            }).collect();
            (Status::Ok, Json(res))
        }
        None => (Status::Ok, Json(vec![])),
    }
}