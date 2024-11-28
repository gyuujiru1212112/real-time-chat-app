# real-time-chat-app
This is the final project for ECE1724H F1 LEC0101 20249:Special Topics in Software Engineering @UofT

## MySQL Database

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


## Client


## Server

### API Endpoints

| Route | Method | Headers | Body Parameters | Return Body |
| -------- | ------- | ------- | ------- | ------- |
| /chatapp/user/signup | POST | N/A | {"username": "", "email": "", "password": ""} | N/A |
| /chatapp/user/login | POST | N/A | {"username": "", "password": ""} | {"message": "Success", "session_id": ""} |
| /chatapp/user/logout | POST | N/A | {"username": "", "session_id": ""} | N/A |
| /chatapp/user/status?username | GET | username,<br>session_id | N/A | "ACTIVE"<br>or "INACTIVE"<br>or "NOT_FOUND" |
| /chatapp/user/allactive | GET | username,<br>session_id | N/A | ["\<user1>", "\<user2>"...] |

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
