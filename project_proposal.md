
# Real-Time Chat Application Project
- Duration: 5 Weeks (development) + 1 Week (demo + buffer time)

- Team Members: Yiduo Jing [1000308142], Kayleigh McNeil [1001278164]

## Motivation

Developing a real-time chat application gives our team the opportunity to explore and practice a wide range of concepts and techniques using Rust.

One of the topics we are interested in is the application of messaging patterns to allow for real-time messaging involving multiple users in a chat room. Based on our initial investigation into approaching this challenge, our plan is to use a publish-subscribe pattern. Applying this messaging pattern to ensure that all users in a chat room receive messages in the same order and in real-time, will be an interesting challenge.

Another area of interest to our group is the application of concurrency in Rust. There will be a need for the use of concurrently for both the server and client components of the application. The application server will need to be able to handle incoming user requests, interact with a database, and run a publish-subscribe messaging service. Concurrency will be needed for the server to be able to handle simultaneous requests from users while ensuring messages continue to be delivered in real time. On the client-side of the application, concurrency will be needed to decouple the sending and receiving of messages so that the publishing of a message does not impact the user's ability to receive messages from others.

Overall, the opportunity to gain experience with messaging patterns and concurrency in Rust, is the primary motivation for this project.


## Objective and Key Features

The overall objective of this project is to create a chat application that allows users to communicate in real-time. There are two types of chats that will be supported: private chats between two users, and chat rooms with many active users at once. In order to achieve real-time messaging and allow for large numbers of users to be supported within a chat room, a publish-subscribe messaging pattern using the Websocket communications protocol will be implemented.


Key features of the chat application are:

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
- [Tentative] A front-end user interface using Yew


## Tentative Plan

The chat application development can be broken down into the client and server components and their key subcomponents.

- Client:
  - CLI for executing the actions described by the section above. This involves making HTTP requests to the chat web server.
  - Chat client for connecting to the pub-sub messaging service.
  - [Tentative] Front-end user interface.
- Server:
  - Web server called by the client application to execute user actions.
  - Database and database connector for executing basic insert and query operations on a MySQL database.
  - Pub-sub messaging service.

Below is a high-level, tentative timeline and breakdown of responsibilities for the development of the chat application.

**Week 1: Set up project, endpoints, and CLI**

Responsibilities:

- Kayleigh McNeil:
  - Configure routes, and define the core API structure.
  - Develop foundational endpoints for authentication and user management (signup, login/out).

- Yiduo Jing:
  - Set up the GitHub repository.
  - Follow the instructions to set up a basic Rocket application with the required dependencies.
  - Create basic CLI commands for signup and logging in/out.
  - Implement the endpoints for private chat/chat channels.

**Week 2: Complete primary API routes, CLI commands, and database scheme design**

Responsibilities:

- Both members:
  - Start to set up MySQL integration, and establish databse tables and models for users, channels and chat history.
  - Design the database scheme.
- Kayleigh McNeil:
  - Create the API endpoints to retrieve users based on the username or userid.
- Yiduo Jing:
  - Develop the CLI commands for joining channels and sending messages.

**Week 3: Establish basic WebSocket connection, real-time messaging, and chat history**

Responsibilities:

- Both members:
  - Set up WebSocket connections for real-time messaging.
  - Develop message broadcasting logic (send and receive messages).
  - Implement CLI commands for retrieving chat history from the API.

**Week 4: Implement presence detection, complete WebSocket integration, and online/offline status**

Responsibilities:

- Kayleigh McNeil:
  - Complete WebSocket integration and start to track the presence detection in the backend.
- Yiduo Jing:
  - Implement functionality to display online/offline status based on WebSocket messages.
  - Implement a CLI command for showing help messages.

**Week 5: Final Testing, bug fixes, optimization**

Responsibilities:

- Kayleigh McNeil:
  - Write unit and integration tests for the back-end services.
  - Optimize the code for performance and scalability.
- Yiduo Jing:
  - Assist in testing and bug fixes.
  - [Tentative] Improve the CLI to a front-end user interface if we have time.

**Week 6: Demo and Final Document**

Responsibilities:

- Both members:
  - Finalize the README.md for the usage and installation instructions.
  - Conduct a video demo of the application, showcasing all the features and functionality.