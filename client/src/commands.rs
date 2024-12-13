use regex::Regex;
use validator::ValidateEmail;

use crate::common::{
    print_password_rule, print_user_name_rule, print_warning_error_msg, CHAT_ROOM_CMD,
    CHECK_USER_STATUS_CMD, EXIT_CMD, HELP_CMD, JOIN_CHAT_ROOM_CMD, LIST_CHAT_ROOMS_CMD,
    LIST_RECIPIENTS_CMD, LIST_USERS_CMD, LOGIN_CMD, LOGOUT_CMD, PRIVATE_CHAT_CMD, RESUME_CHAT_CMD,
    SIGNUP_CMD,
};

#[derive(Debug)]
pub enum Command {
    Signup {
        username: String,
        email: String,
        password: String,
    },
    Login {
        username: String,
        password: String,
    },
    Logout,
    ListUsers,
    CheckUserStatus {
        username: String, // the user that we want to check
    },
    CreatePrivateChat {
        with_user: String,
    },
    ResumeChat {
        with_user: String,
    },
    CreateChatRoom {
        name: String,
        users: Vec<String>,
    },
    JoinChatRoom {
        chat_id: String,
    },
    ListAllChatRooms,
    ListAllRecipients,
    Help,
    Quit,
}

pub fn parse_command(input: &str) -> Option<Command> {
    let input_list: Vec<&str> = input.trim().split_whitespace().collect();
    match input_list.as_slice() {
        [SIGNUP_CMD, username, email, password] => Some(Command::Signup {
            username: (username.to_string()),
            email: (email.to_string()),
            password: (password.to_string()),
        }),
        [LOGIN_CMD, username, password] => Some(Command::Login {
            username: (username.to_string()),
            password: (password.to_string()),
        }),
        [LOGOUT_CMD] => Some(Command::Logout),
        [LIST_USERS_CMD] => Some(Command::ListUsers),
        [CHECK_USER_STATUS_CMD, username] => Some(Command::CheckUserStatus {
            username: (username.to_string()),
        }),
        [PRIVATE_CHAT_CMD, with_user] => Some(Command::CreatePrivateChat {
            with_user: (with_user.to_string()),
        }),
        [RESUME_CHAT_CMD, with_user] => Some(Command::ResumeChat {
            with_user: (with_user.to_string()),
        }),
        [CHAT_ROOM_CMD, name, users @ ..] => {
            let user_list: Vec<String> = users.iter().map(|&user| user.to_string()).collect();
            Some(Command::CreateChatRoom {
                name: (name.to_string()),
                users: (user_list),
            })
        }
        [JOIN_CHAT_ROOM_CMD, chat_id] => Some(Command::JoinChatRoom {
            chat_id: chat_id.to_string(),
        }),
        [LIST_CHAT_ROOMS_CMD] => Some(Command::ListAllChatRooms),
        [LIST_RECIPIENTS_CMD] => Some(Command::ListAllRecipients),
        [HELP_CMD] => Some(Command::Help),
        [EXIT_CMD] => Some(Command::Quit),
        _ => None,
    }
}

pub fn is_valid_email_addr(email: &str) -> bool {
    let res = email.validate_email();
    if !res {
        println!("Error: Invalid email address.");
    }
    res
}

pub fn is_valid_username(username: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9._]{4,19}[a-zA-Z0-9]$").unwrap();
    let res = re.is_match(username);

    if !res {
        print_warning_error_msg("Error: Invalid username.");
        print_user_name_rule();
    }
    res
}

pub fn is_valid_password(password: &str) -> bool {
    // 6 - 16
    if password.len() < 6 || password.len() > 16 {
        print_warning_error_msg("Error: Invalid password.");
        print_password_rule();
        return false;
    }

    // at least one digit
    let has_digit = Regex::new(r"[0-9]").unwrap();
    if !has_digit.is_match(password) {
        print_warning_error_msg("Error: Invalid password.");
        print_password_rule();
        return false;
    }

    // at least one special character
    let has_special = Regex::new(r"[!@#$%^&*]").unwrap();
    if !has_special.is_match(password) {
        print_warning_error_msg("Error: Invalid password.");
        print_password_rule();
        return false;
    }

    // all characters are valid
    let valid_chars = Regex::new(r"^[a-zA-Z0-9!@#$%^&*]+$").unwrap();
    if !valid_chars.is_match(password) {
        print_warning_error_msg("Error: Invalid password.");
        print_password_rule();
        return false;
    }

    true
}
