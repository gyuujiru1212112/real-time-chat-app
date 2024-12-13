use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post};

use crate::database::DbManager;

use super::common::{is_session_id_valid, UserReqInfo};

#[derive(Deserialize, Serialize)]
pub struct ChatRoomResponse {
    id: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatRoomRequest {
    username: String,
    session_id: String,
    room_name: String,
    members: Vec<String>, // other than the username itself
}

#[derive(Deserialize, Serialize)]
pub struct PrivateChatRequest {
    username: String,
    session_id: String, // user's session id
    recipient: String,
}

#[post("/private-chat/create", format = "json", data = "<private_chat_info>")]
pub async fn create_private_chat<'a>(
    private_chat_info: Json<PrivateChatRequest>,
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<String>) {
    let valid_session: bool = is_session_id_valid(
        &private_chat_info.username,
        &private_chat_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return (Status::Unauthorized, Json(String::new()));
    }

    let res = db_manager
        .insert_private_chat(&private_chat_info.username, &private_chat_info.recipient)
        .await;

    if let Some(id) = res {
        println!(
            "Private chat created between users '{}' and '{}'",
            &private_chat_info.username, &private_chat_info.recipient
        );
        return (Status::Created, Json(id));
    } else {
        println!(
            "Failed to create a private chat between users '{}' and '{}'",
            &private_chat_info.username, &private_chat_info.recipient
        );

        return (Status::Unauthorized, Json(String::new()));
    }
}

#[post("/private-chat/resume", format = "json", data = "<private_chat_info>")]
pub async fn resume_private_chat<'a>(
    private_chat_info: Json<PrivateChatRequest>,
    db_manager: &rocket::State<DbManager>,
) -> Status {
    let valid_session: bool = is_session_id_valid(
        &private_chat_info.username,
        &private_chat_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return Status::Unauthorized;
    }

    Status::Ok
}

#[post("/private-chat/exit", format = "json", data = "<private_chat_info>")]
pub async fn exit_private_chat<'a>(
    private_chat_info: Json<PrivateChatRequest>,
    db_manager: &rocket::State<DbManager>,
) -> Status {
    let valid_session: bool = is_session_id_valid(
        &private_chat_info.username,
        &private_chat_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return Status::Unauthorized;
    }

    Status::Ok
}

#[post("/chat-room/create", format = "json", data = "<chat_room_info>")]
pub async fn create_chat_room<'a>(
    chat_room_info: Json<ChatRoomRequest>,
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<String>) {
    let valid_session: bool = is_session_id_valid(
        &chat_room_info.username,
        &chat_room_info.session_id,
        db_manager.inner(),
    )
    .await;

    if !valid_session {
        return (Status::Unauthorized, Json(String::new()));
    }

    let members = {
        let mut temp = chat_room_info.members.clone();
        temp.push(chat_room_info.username.to_owned());
        temp
    };
    let res = db_manager
        .insert_chat_room(&chat_room_info.room_name, &members)
        .await;
    if let Some(chat_room_id) = res {
        println!("Chat room '{}' created", chat_room_info.room_name);
        (Status::Created, Json(chat_room_id))
    } else {
        println!("Failed to create chat room '{}'.", chat_room_info.room_name);
        (Status::InternalServerError, Json(String::new()))
    }
}

#[post("/chat-room/join", format = "json", data = "<chat_room_info>")]
pub async fn joint_chat_room<'a>(
    chat_room_info: Json<ChatRoomRequest>,
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

    match db_manager
        .get_all_chat_recipients(&user_info.username)
        .await
    {
        Some(partners) => (Status::Ok, Json(partners)),
        None => (Status::Ok, Json(vec![])),
    }
}

#[get("/chat-room/all")]
pub async fn get_all_chat_rooms(
    user_info: UserReqInfo,
    db_manager: &rocket::State<DbManager>,
) -> (Status, Json<Vec<ChatRoomResponse>>) {
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
            let res: Vec<ChatRoomResponse> = rooms
                .iter()
                .map(|room| ChatRoomResponse {
                    id: room.0.clone(),
                    name: room.1.clone(),
                })
                .collect();
            (Status::Ok, Json(res))
        }
        None => (Status::Ok, Json(vec![])),
    }
}
