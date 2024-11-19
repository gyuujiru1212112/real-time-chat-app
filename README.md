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
    user_id int AUTO_INCREMENT PRIMARY KEY,
    username varchar(255) NOT NULL,
    email varchar(255),
    password varchar(255) NOT NULL
);
```


## Client


## Server
