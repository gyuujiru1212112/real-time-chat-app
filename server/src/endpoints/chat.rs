use rocket::http::Status;
use rocket::{get, post};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::database::DbManager;

use super::common::{is_session_id_valid, UserReqInfo};

#[derive(Deserialize, Serialize)]
pub struct ChatRoom {
    id: String,
    name: String
}

#[derive(Deserialize, Serialize)]
pub struct ChatRoomInfo {
    username: String,
    session_id: String,
    room_name: String,
    members: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PrivateChatInfo {
    username: String,
    session_id: String,
    user2: String
}

#[post("/private-chat/create", format = "json", data = "<private_chat_info>")]
pub async fn create_private_chat<'a>(
    private_chat_info: Json<PrivateChatInfo>,
    db_manager: &rocket::State<DbManager>) 
    -> Status
{
    let valid_session: bool = is_session_id_valid(
        &private_chat_info.username,
        &private_chat_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return Status::Unauthorized;
    }

    let success: bool = db_manager
        .insert_private_chat(
            &private_chat_info.username,
            &private_chat_info.user2
        ).await;

        if success {
            println!("Private chat created between users '{}' and '{}'",
                &private_chat_info.username, &private_chat_info.user2);
            Status::Created
        } else {
            println!("Failed to create a private chat between users '{}' and '{}'",
                &private_chat_info.username, &private_chat_info.user2);
            Status::InternalServerError
        }

}

#[post("/private-chat/resume", format = "json", data = "<private_chat_info>")]
pub async fn resume_private_chat<'a>(
    private_chat_info: Json<PrivateChatInfo>,
    db_manager: &rocket::State<DbManager>,) 
    -> Status
{
    let valid_session: bool = is_session_id_valid(
        &private_chat_info.username,
        &private_chat_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return Status::Unauthorized;
    }
Status::Created

}

#[post("/chat-room/create", format = "json", data = "<chat_room_info>")]
pub async fn create_chat_room<'a>(
    chat_room_info: Json<ChatRoomInfo>,
    db_manager: &rocket::State<DbManager>,
) -> Status {
    let valid_session: bool = is_session_id_valid(
        &chat_room_info.username,
        &chat_room_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return Status::Unauthorized;
    }

    let success: bool = db_manager
        .insert_chat_room(
            &chat_room_info.room_name,
            &chat_room_info.members
        )
        .await;
    if success {
        println!("Chat room '{}' created", chat_room_info.room_name);
        Status::Created
    } else {
        println!("Failed to create chat room '{}'.", chat_room_info.room_name);
        Status::InternalServerError
    }
}

#[post("/chatroom/join", format = "json", data = "<chat_room_info>")]
pub async fn joint_chat_room<'a>(
    chat_room_info: Json<ChatRoomInfo>,
    db_manager: &rocket::State<DbManager>,
) -> Status {
    let valid_session: bool = is_session_id_valid(
        &chat_room_info.username,
        &chat_room_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return Status::Unauthorized;
    }

    Status::Created
}

#[get("/private-chat/recipients")]
pub async fn get_all_recipients(
    user_info: UserReqInfo,
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<Vec<String>>) {
    let valid_session: bool = is_session_id_valid(
        &user_info.username,
        &user_info.session_id,
        db_manager.inner(),
    )
    .await;
    if !valid_session {
        return (Status::Unauthorized, Json(vec![]));
    }

    match db_manager.get_all_chat_recipients(&user_info.username).await {
        Some(partners) => {
            (Status::Ok, Json(partners))
        }
        None => (Status::Ok, Json(vec![])),
    }
}

#[get("/chatroom/all")]
pub async fn get_all_chat_rooms(
    user_info: UserReqInfo,
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<Vec<ChatRoom>>) {
    let valid_session: bool = is_session_id_valid(
        &user_info.username,
        &user_info.session_id,
        db_manager.inner(),
    )
    .await;
    if !valid_session {
        return (Status::Unauthorized, Json(vec![]));
    }

    match db_manager.get_all_chat_rooms().await {
        Some(rooms) => {
            let res: Vec<ChatRoom> = rooms.iter().map(|room| {
                ChatRoom {
                    id: room.0.clone(),
                    name: room.1.clone(),
                }
            }).collect();
            (Status::Ok, Json(res))
        }
        None => (Status::Ok, Json(vec![])),
    }
}