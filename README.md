# Real-Time Chat Application

## Team Members:
- Kayleigh McNeil [1001278164]
  - email: kayleigh.mcneil@mail.utoronto.ca
- Yiduo Jing [1000308142]
  - email: yiduo.jing@mail.utoronto.ca

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
This command-line interface is built using the rustyline crate, **enabling command history navigation with the up/down arrow keys**. It also supports **copying and pasting** text into the command line input.

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
  Initiates a private chat with the specified user. A private chat cannot be created if one already exists with the same participants, regardless of the order. Use `list-recipients` to view all recipients, and `resume-chat [recipient]` to continue an existing private chat.
- **`resume-chat [recipient]`**  
  Resumes an ongoing private chat with the specified recipient.
- **`list-recipients`**  
  Lists all the users you have had private chats with.
- **`chat-room [group_name] [user1] [user2] [user3]...`**  
  Creates a new chat room with the specified group name and users.
- **`join-chat-room [id]`**  
  Joins an existing chat room by its ID. Use `list-chat-rooms` to view the ID.
- **`list-chat-rooms`**  
  Lists all existing chat rooms with their associated names and IDs.
- **`exit`**  
  Exits the program.

#### Commands available inside a chat session
  - **`:help`**   
  Show chat command options.
  - **`:exit`**    
  Leave the chat and return to main app command line. Press the `Enter` key if it becomes unresponsive.

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

### Pub-Sub Messaging Service
The messaging service of the application uses a publisher-subscriber messaging pattern. The WebSocket communications protocol is used for communication between publishers/subscribers and the messaging server.

The pub-sub messaging service is made up of the following main components:
* **Server** - The messaging server that starts up the TCP listener, accepts and handles new connections, and uses the broker to route messages.
* **Broker** - Keeps track of existing subscribers and the topics they are subscribed to. The broker is responsible for routing messages received by the server to the appropriate subscribers. The broker also uses the database manager to validate user sessions when a new subscription request is received to ensure only active, valid users are able to subscribe to topics.
* **DB Manager** - A database manager for handling a connection to the MySQL db for the purpose of validating user sessions when a subscription message from a user is received by the server.
* **Client** - A module that can be used by other rust modules to connect to the messaging server, subscribe to topics, and send and receive messages.

### MySQL Database

There are four tables used as part of this application for keeping a record of users and chats. The SQL commands used to create these tables can be found in the `mysql/dump.sql` file in this repository.

Tables
| Table Name | Description |
|------------|-------------|
| user | Contains an entry for each user. Users must have unique usernames. When a user is logged in, the active session_id for that user is stored in this table for verification purposes. |
| private_chat | A record of the existing private chats that exist between pairs of users and their unique chat ids. |
| chat_room | A record of the different chat rooms that exist and their associated names and chat unique ids. |
| room_member | A record of the team members of the chat room with their associated names and chat_room ids. |

## Reproducibility Guide:

### Build & Run

1. Install Docker and Docker Compose if not already installed
2. Navigate to the root directory of this project (`real-time-chat-app/`)
3. Build and run the server-side components using docker-compose:
  ```
  docker-compose build
  docker-compose up -d
  ```
  * This will start up three docker containers running in the background: `mysqldb`, `chatapp_server`, and `chatapp_pubsub`.
4. Run one or more instances of the client to interact with the application: `cargo run --release -p client`


## Contributions by each team member:
- **Kayleigh McNeil**:
  - Developed initial server setup with Rocket and sqlx.
  - Designed and developed the pub-sub messaging service using tokio and tokio_websockets.
  - Integrated the pub-sub messaging service into the applicaiton.
  - Dockerized the server, pub-sub service, and mysql db components.
  - Contributed to the project report.
- **Yiduo Jing**:
  - Developed the initial client setup.
  - Designed the CLI utility with rustyline for user interaction.
  - Implemented CLI commands for features such as signup, login, logout, listing all users, checking user status, initiating/resuming private chats, creating chat rooms, listing all recipients, and listing all chat rooms.
  - Built API endpoints with Rocket and sqlx to support functionalities like listing users, initiating/resuming private chats, creating chat rooms, and retrieving chat room or recipient lists.
  - Contributed to the project report.

## Lessons learned and concluding remarks
**Lessons Learned**
- Developed a strong knowledge of real-time communication by building a messaging service using the publish-subscribe model with WebSockets.
- Gained skills in using Rocket to create API endpoints and handle routing efficiently.
- Earned experience using sqlx to design and work with a MySQL database service.

The project related to the real-time chat application was full of challenges and rewarding experiences. It allowed us to apply the knowledge from the lectures in practice. Throughout this project, we deepened our understanding of real-time systems, user authentication, and the complexities of developing scalable and user-friendly applications.

While the completed project meets the essential requirements, there are areas where we can improve. Potential enhancements include increasing performance, implementing encryption for secure communication, and improving the user interface to a front-end application. Overall, this project has taught us valuable lessons and equipped us with a deeper understanding of complex systems.
