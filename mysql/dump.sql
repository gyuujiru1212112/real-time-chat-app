CREATE TABLE user (
    username varchar(255) NOT NULL,
    email varchar(255),
    password varchar(255) NOT NULL,
    session_id varchar(255),
    PRIMARY KEY (username)
);
CREATE TABLE private_chat (
    chat_id VARCHAR(255),
    user1 VARCHAR(255) NOT NULL,
    user2 VARCHAR(255) NOT NULL,
    FOREIGN KEY (user1) REFERENCES user(username) ON DELETE CASCADE,
    FOREIGN KEY (user2) REFERENCES user(username) ON DELETE CASCADE,
    UNIQUE (user1, user2)
);
CREATE TABLE chat_room (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    chat_room_id varchar(255),
    name VARCHAR(100) NOT NULL
);
CREATE TABLE room_member (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    room_id BIGINT NOT NULL,
    username VARCHAR(255)  NOT NULL,
    FOREIGN KEY (room_id) REFERENCES chat_room(id) ON DELETE CASCADE,
    FOREIGN KEY (username) REFERENCES user(username) ON DELETE CASCADE,
    UNIQUE (room_id, username)
);
CREATE TABLE chat_message (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    chat_id VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    message VARCHAR(255),
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (username) REFERENCES user(username) ON DELETE CASCADE
);
