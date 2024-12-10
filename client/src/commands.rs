use validator::ValidateEmail;
use regex::Regex;

use crate::common::{print_password_rule, print_user_name_rule, print_warning_error_msg};

#[derive(Debug)]
pub enum Command {
    Signup {
        username: String,
        email: String,
        password: String
    },
    Login {
        username: String,
        password: String
    },
    Logout,
    ListActiveUsers,
    CheckUserStatus {
        username: String // the user that we want to check
    },
    CreatePrivateChat {
        with_user: String // user1 should be the current user
    },
    CreateChatRoom {
        name: String,
        users: Vec<String> // current user included
    },
    ListAllChatRooms,
    Help,
    Quit
}

pub fn parse_command(input: &str) -> Option<Command>
{
    let input_list: Vec<&str> = input.trim().split_whitespace().collect();
    match input_list.as_slice() {
        ["signup", username, email, password] => Some(
            Command::Signup { username: (username.to_string()), email: (email.to_string()), password: (password.to_string()) }),
        ["login", username, password] => Some(
            Command::Login { username: (username.to_string()), password: (password.to_string()) }),
        ["logout"] => Some(Command::Logout),
        ["list-users"] => Some(Command::ListActiveUsers),
        ["check", username] => Some(
            Command::CheckUserStatus { username: (username.to_string()) }
        ),
        ["private-chat", with_user] => Some(Command::CreatePrivateChat { with_user: (with_user.to_string()) }),
        ["chat-room", name, users @..] => {
            let user_list: Vec<String> = users.iter().map(|&user| user.to_string()).collect();
            Some(Command::CreateChatRoom { name: (name.to_string()), users: (user_list) })
        },
        ["list-chat-rooms"] => Some(Command::ListAllChatRooms),
        ["help"] => Some(Command::Help),
        ["exit"] => Some(Command::Quit),
        _ => None
    }
}

pub fn is_valid_email_addr(email: &str) -> bool
{
    let res = email.validate_email();
    if !res
    {
        println!("Error: Invalid email address.");
    }
    res
}

pub fn is_valid_username(username: &str) -> bool
{
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9._]{4,19}[a-zA-Z0-9]$").unwrap();
    let res = re.is_match(username);

    if !res
    {
        print_warning_error_msg("Error: Invalid username.");
        print_user_name_rule();
    }
    res
}

pub fn is_valid_password(password: &str) -> bool
{
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