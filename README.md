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
- Commands
  - signup [username] [email] [password]
  - login [username] [password]
  - logout
  - help
  - exit
- Rules
  - The username must satisfy:
    1. Number of characters must be between 8 and 20
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

## Server
