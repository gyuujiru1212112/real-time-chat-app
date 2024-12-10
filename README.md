# real-time-chat-app
This is the final project for ECE1724H F1 LEC0101 20249:Special Topics in Software Engineering @UofT
## Team Members:
Kayleigh McNeil [1001278164], Yiduo Jing [1000308142]
Contact: yiduo.jing@mail.utoronto.ca

## Motivation and Objectives

Developing a real-time chat application offers our team an exciting opportunity to deepen our understanding and practice a range of advanced concepts and techniques in Rust. The project's primary focus is to design a robust system capable of supporting real-time communication, which will require the application of messaging patterns and concurrency principles—two areas we are eager to explore.  

One of the key technical challenges is implementing a **publish-subscribe messaging pattern** to ensure all users in a chat room receive messages in the same order and in real time. This pattern is well-suited for handling messaging involving multiple users in a dynamic and scalable manner, making it critical for supporting large chat rooms effectively.  

Another important aspect is the use of **concurrency in Rust**. On the server side, concurrency will enable the handling of simultaneous user requests, database interactions, and the seamless operation of the publish-subscribe messaging service. On the client side, concurrency will allow users to send and receive messages simultaneously, ensuring that sending a message does not interrupt the ability to receive messages from others.  

The overall objective of this project is to create a **real-time chat application** that supports two types of communication:  
1. **Private chats** between two users.  
2. **Chat rooms** where multiple users can participate simultaneously.  

To achieve this, the application will employ the **WebSocket communication protocol** to enable real-time messaging and incorporate the publish-subscribe pattern to maintain message consistency and delivery order. By undertaking this project, we aim to gain hands-on experience with real-world challenges in messaging systems, concurrency, and Rust's ecosystem, all while delivering a functional and efficient chat application.  

## Features:
- The ability to create new users
- Authenticate users with a simple username and password combination
- A command-line interface that allows a user to execute actions when running the application
- Users will have the ability to execute the following command-line actions:
  - Sign up to create a new user
  - Show help messages
  - Login and logout
  - View a list of other users with an "active" status
  - Check the activity status of a specific user based on their username
  - Create a new private chat
  - Resume an existing private chat
  - Create a new chat room
  - Join an existing chat room
  - List existing chat rooms
  - Exit from a private chat or chat room
  - Send and receive messages in real-time in:
    - A private chat between two users
    - A chat room with many active users
- View chat history when resuming a private chat or joining an existing chat room
- A publish-subscribe messaging service that clients connect to using Websockets for bidirectional communication of messages

## User’s (or Developer’s) Guide: How does a user — or developer, if the project is a crate — use each of the main features in the project deliverable?

## Reproducibility Guide:

### MySQL Database

(Adding some quick notes here for now. Will tidy them up later.)

* Install MySQL with homebrew and configure a root user and password
* Start up the service: `brew services start mysql`
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
    id INT AUTO_INCREMENT PRIMARY KEY,
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
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);
```
* Create the `room_member` table:
```
CREATE TABLE room_member (
    id INT AUTO_INCREMENT PRIMARY KEY,
    room_id INT NOT NULL,
    username VARCHAR(255)  NOT NULL,
    FOREIGN KEY (room_id) REFERENCES chat_room(id) ON DELETE CASCADE,
    FOREIGN KEY (username) REFERENCES user(username) ON DELETE CASCADE,
    UNIQUE (room_id, username)
);
```


### Client
- Start the client program: `cargo run -p client`
- Commands available inside the program:
  - Show help message: `help`
  - Signup: `signup [username] [email] [password]`
  - Login: `login [username] [password]`
  - Logout: `logout`
  - List all the active users: `list-all`
  - Check the status based on username: `check [username]`
  - Quit the program: `exit`

- Rules:
  - The username must satisfy:
    1. Number of characters must be between 5 and 20
    2. Must start with a letter.
    3. Only alphanumeric characters (a-z, A-Z, 0-9), underscores (_) and dots (.) are allowed.
    4. No consecutive underscores or dots (e.g., __ or ..).
    5. No underscore or dot at the end.
  - The password must satisfy:
    1. Number of characters must be between 6 and 16.
    2. Must contain:
        - At least one digit (0-9).
        - At least one special character (!@#$%^&*).
    3. Can only contain:
       - Letters (a-z, A-Z).
       - Digits (0-9).
       - Special characters (!@#$%^&*).

### Server

#### API Endpoints

| Route | Method | Headers | Body Parameters | Return Body |
| -------- | ------- | ------- | ------- | ------- |
| /chatapp/user/signup | POST | N/A | {"username": "", "email": "", "password": ""} | N/A |
| /chatapp/user/login | POST | N/A | {"username": "", "password": ""} | {"message": "Success", "session_id": ""} |
| /chatapp/user/logout | POST | N/A | {"username": "", "session_id": ""} | N/A |
| /chatapp/user/status?username | GET | username,<br>session_id | N/A | "ACTIVE"<br>or "INACTIVE"<br>or "NOT_FOUND" |
| /chatapp/user/allactive | GET | username,<br>session_id | N/A | ["\<user1>", "\<user2>"...] |
| /chatapp/chat/private-chat | POST | N/A | {"user1":"", "user2":""} | N/A |
| /chatapp/chat/chat-room | POST | N/A | {"name":"", "users":["", ""]} | N/A |

Sample Curl Requests

- /chatapp/user/signup:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/signup' --header 'Content-Type: application/json' --data '{"username": "test_user", "email": "test.user@gmail.com", "password": "testpwd"}'`
- /chatapp/user/login:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/login' --header 'Content-Type: application/json' --data '{"username": "test_user", "password": "testpwd"}'`
- /chatapp/user/logout:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/logout' --header 'Content-Type: application/json' --data '{"username": "test_user", "session_id": "f043ab79-032c-43d6-957e-6b78241632bf"}'`
- /chatapp/user/status?username:  
    `curl --location 'http://127.0.0.1:8000/chatapp/user/status?username=test_user2' --header 'username: test_user' --header 'session_id: f043ab79-032c-43d6-957e-6b78241632bf'`
- /chatapp/user/allactive:
    `curl --location 'http://127.0.0.1:8000/chatapp/user/allactive' --header 'username: test_user' --header 'session_id: f043ab79-032c-43d6-957e-6b78241632bf'`
- /chatapp/chat/private_chat:
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/private-chat' --header 'Content-Type: application/json' --data '{"user1": "ydjing121", "user2":"jingyidu122"}'`
- /chatapp/chat/chat_room:
    `curl --location 'http://127.0.0.1:8000/chatapp/chat/chat-room' --header 'Content-Type: application/json' --data '{"name": "group1", "users":["ydjing121", "jingyidu122"]}'`



## Contributions by each team member:
Kayleigh McNeil:
Yiduo Jing:


## Lessons learned and concluding remarks: Write about any lessons the team has learned throughout the project and concluding remarks, if any.
