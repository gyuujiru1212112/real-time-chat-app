# Real-Time Chat Application
*This is the final project for ECE1724: Special Topics in Software Engineering @ UofT*

## Team Members
- **Kayleigh McNeil** [1001278164]
  - Email: 
- **Yiduo Jing** [1000308142]
  - Email: yiduo.jing@mail.utoronto.ca

## Motivation and Objectives
Developing a real-time chat application offers our team an exciting opportunity to deepen our understanding and practice a range of advanced concepts and techniques in Rust. The project's primary focus is to design a robust system capable of supporting real-time communication, which will require the application of messaging patterns and concurrency principles—two areas we are eager to explore.

One of the key technical challenges is implementing a **publish-subscribe messaging pattern** to ensure all users in a chat room receive messages in the same order and in real time. This pattern is well-suited for handling messaging involving multiple users in a dynamic and scalable manner, making it critical for supporting large chat rooms effectively.

Another important aspect is the use of **concurrency in Rust**. On the server side, concurrency will enable the handling of simultaneous user requests, database interactions, and the seamless operation of the publish-subscribe messaging service. On the client side, concurrency will allow users to send and receive messages simultaneously, ensuring that sending a message does not interrupt the ability to receive messages from others.

The overall objective of this project is to create a **real-time chat application** that supports two types of communication:
1. **Private chats** between two users.
2. **Chat rooms** where multiple users can participate simultaneously.

To achieve this, the application will employ the **WebSocket communication protocol** to enable real-time messaging and incorporate the publish-subscribe pattern to maintain message consistency and delivery order. By undertaking this project, we aim to gain hands-on experience with real-world challenges in messaging systems, concurrency, and Rust's ecosystem, all while delivering a functional and efficient chat application.

## Features
- Create new users
- Authenticate users with a simple username and password combination
- Command-line interface (CLI) for user actions
- User actions:
  - Sign up to create a new user
  - View help messages
  - Login and logout
  - View a list of other users with their status
  - Check the activity status of a specific user
  - Create a new private chat
  - Resume an existing private chat
  - List all recipients with whom the user has a private chat
  - Create a new chat room
  - Join an existing chat room
  - List existing chat rooms
  - Exit from a private chat or chat room
  - Send and receive messages in real-time:
    - In a private chat between two users
    - In a chat room with many active users
- View chat history when resuming a private chat or joining an existing chat room
- Publish-subscribe messaging service with WebSocket for bidirectional communication

## User’s Guide

### Client (Command-line Utility)
This command-line interface is built using the rustyline crate, enabling command history navigation with the up/down arrow keys. It also supports copying and pasting text into the command line input.

#### Before Login
- **Signup & Login Rules:**
  - **Username Requirements:**
    1. Length must be between 5 and 20 characters.
    2. Must start with a letter.
    3. Only alphanumeric characters (a-z, A-Z, 0-9), underscores (_), and dots (.) are allowed.
    4. No consecutive underscores or dots (e.g., `__` or `..`).
    5. Cannot end with an underscore or dot.
  - **Password Requirements:**
    1. Length must be between 6 and 16 characters.
    2. Must contain at least one digit (0-9) and at least one special character (!@#$%^&*).
    3. Can only contain letters (a-z, A-Z), digits (0-9), and special characters (!@#$%^&*).

#### Commands Available Before Login
- **`help`**  
  Displays the help message, showing available commands.
- **`signup [username] [email] [password]`**  
  Signs up a new account with the provided username, email, and password.
- **`login [username] [password]`**  
  Logs in with the provided username and password.
- **`exit`**  
  Exits the program.

#### After Login
Once logged in, the following commands are available:
- **`logout`**  
  Logs out from the current session.
- **`list-users`**  
  Lists all users in the system.
- **`check [username]`**  
  Checks the online status of a specific user.
- **`private-chat [with_user_name]`**  
  Initiates a private chat with the specified user.
- **`resume-chat [recipient]`**  
  Resumes an ongoing private chat with the specified recipient.
- **`list-recipients`**  
  Lists all the users you have had private chats with.
- **`chat-room [group_name] [user1] [user2] [user3]...`**  
  Creates a new chat room with the specified group name and users.
- **`join-chat-room [id]`**  
  Joins an existing chat room by its ID.
- **`list-chat-rooms`**  
  Lists all existing chat rooms.
- **`exit`**  
  Exits the program.

### Server

#### API Endpoints

| Route | Method | Headers | Body Parameters | Return Body |
|-------|--------|---------|------------------|--------------|
| /chatapp/user/signup | POST | N/A | {"username": "", "email": "", "password": ""} | N/A |
| /chatapp/user/login | POST | N/A | {"username": "", "password": ""} | {"message": "Success", "session_id": ""} |
| /chatapp/user/logout | POST | N/A | {"username": "", "session_id": ""} | N/A |
| /chatapp/user/status?username | GET | username,<br>session_id | N/A | "ACTIVE" or "INACTIVE" or "NOT_FOUND" |
| /chatapp/user/allusers | GET | username,<br>session_id | N/A | [{"username":"user1","status":""},{"username":"user2","status":""},{"username":"user3","status":""}...] |
| /chatapp/chat/private-chat/create | POST | N/A | {"username":"", "session_id":"", "recipient":""} | chat_id |
| /chatapp/chat/private-chat/resume | POST | N/A | {"username":"", "session_id":"", "recipient":""} | chat_id |
| /chatapp/chat/chat-room/create | POST | N/A | {"username":"", "session_id":"", "room_name":"", "members": ["", "", ""]} | chat_room_id |
| /chatapp/chat/chat-room/all | GET | username,<br>session_id | N/A | ["room_id1", "room_id2", "room_id3"] |
| /chatapp/chat/private-chat/recipients | GET | username,<br>session_id | N/A | ["recipient1", "recipient2"] |

#### Sample Curl Requests

- /chatapp/user/signup:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/signup' --header 'Content-Type: application/json' --data '{"username": "test_user", "email": "test.user@gmail.com", "password": "testpwd"}'`
- /chatapp/user/login:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/login' --header 'Content-Type: application/json' --data '{"username": "test_user", "password": "testpwd"}'`
- /chatapp/user/logout:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/logout' --header 'Content-Type: application/json' --data '{"username": "test_user", "session_id": "f043ab79-032c-43d6-957e-6b78241632bf"}'`
- /chatapp/user/status?username:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/status?username=test_user2' --header 'username: test_user' --header 'session_id: f043ab79-032c-43d6-957e-6b78241632bf'`
- /chatapp/user/allusers:    
    `curl --location 'http://127.0.0.1:8000/chatapp/user/allusers' --header 'username: test_user' --header 'session_id: f043ab79-032c-43d6-957e-6b78241632bf'`
- /chatapp/chat/private-chat/create:    
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/private-chat/create' --header 'Content-Type: application/json' --data '{"username": "test_user", "session_id": "f043ab79-032c-43d6-957e-6b78241632bf", "recipient": "test_user2"}'`
- /chatapp/chat/private-chat/resume:    
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/private-chat/resume' --header 'Content-Type: application/json' --data '{"username": "test_user", "session_id": "f043ab79-032c-43d6-957e-6b78241632bf", "recipient": "test_user2"}'`
- /chatapp/chat/chat-room/create:    
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/chat-room/create' --header 'Content-Type: application/json' --data '{"username": "test_user", "session_id": "92458410-2077-4ef3-a7c8-0be76c6122bb", "room_name": "group1", "members": ["test_user1", "test_user2"...]}'`
- /chatapp/chat/chat-room/all:    
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/chat-room/all' --header 'username: test_user' --header 'session_id: f043ab79-032c-43d6-957e-6b78241632bf'`
- /chatapp/chat/private-chat/recipients:    
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/private-chat/recipients' --header 'username: test_user' --header 'session_id: f043ab79-032c-43d6-957e-6b78241632bf'`

## Reproducibility Guide:

### Environment Setup
(Adding some quick notes here for now. Will tidy them up later.)

* **Install MySQL**: homebrew and configure a root user and password
* **Start up the service**: `brew services start mysql`
* `mysql -u root -p`
* Create the db for the app and create a user to be used by the app when connecting to the db:
```
CREATE DATABASE chatapp;
CREATE USER ‘chatserver’@‘localhost' IDENTIFIED BY ‘ServerPass123’;
GRANT ALL ON chatapp.* TO 'chatserver'@'localhost';
```
* Create the `user` table:
```
CREATE TABLE user (
    username varchar(255) NOT NULL,
    email varchar(255),
    password varchar(255) NOT NULL,
    session_id varchar(255),
    PRIMARY KEY (username)
);
```
* Create the `private_chat` table:
```
CREATE TABLE private_chat (
    chat_id VARCHAR(255),
    user1 VARCHAR(255) NOT NULL,
    user2 VARCHAR(255) NOT NULL,
    FOREIGN KEY (user1) REFERENCES user(username) ON DELETE CASCADE,
    FOREIGN KEY (user2) REFERENCES user(username) ON DELETE CASCADE,
    UNIQUE (user1, user2)
);
```
* Create the `chat_room` table:
```
CREATE TABLE chat_room (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    chat_room_id varchar(255),
    name VARCHAR(100) NOT NULL
);
```
* Create the `room_member` table:
```
CREATE TABLE room_member (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    room_id BIGINT NOT NULL,
    username VARCHAR(255)  NOT NULL,
    FOREIGN KEY (room_id) REFERENCES chat_room(id) ON DELETE CASCADE,
    FOREIGN KEY (username) REFERENCES user(username) ON DELETE CASCADE,
    UNIQUE (room_id, username)
);
```
### Build & Run
* **Start the server first** (only one instance): `cargo run -p server`
* **Run each client** (multiple instances allowed): `cargo run -p client`



## Contributions by each team member:
- **Kayleigh McNeil**:
- **Yiduo Jing**:
  - Rocket application & client setup
  - CLI commands for
  - API endpoints for 
  - Report


## Lessons learned and concluding remarks: Write about any lessons the team has learned throughout the project and concluding remarks, if any.