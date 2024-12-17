use colored::Colorize;

pub const SIGNUP_CMD: &str = "signup";
pub const LOGIN_CMD: &str = "login";
pub const LOGOUT_CMD: &str = "logout";
pub const LIST_USERS_CMD: &str = "list-users";
pub const CHECK_USER_STATUS_CMD: &str = "check";
pub const PRIVATE_CHAT_CMD: &str = "private-chat";
pub const RESUME_CHAT_CMD: &str = "resume-chat";
pub const CHAT_ROOM_CMD: &str = "chat-room";
pub const LIST_CHAT_ROOMS_CMD: &str = "list-chat-rooms";
pub const JOIN_CHAT_ROOM_CMD: &str = "join-chat-room";
pub const LIST_RECIPIENTS_CMD: &str = "list-recipients";
pub const HELP_CMD: &str = "help";
pub const EXIT_CMD: &str = "exit";

pub fn print_user_name_rule() {
    let rule = r#"
    The username must satisfy:
    1. Number of characters must be between 5 and 20.
    2. Must start with a letter.
    3. Only alphanumeric characters (a-z, A-Z, 0-9), underscores (_) and dots (.) are allowed.
    4. No consecutive underscores or dots (e.g., __ or ..).
    5. No underscore or dot at the end.
    "#;
    println!("{}", rule.red());
}

pub fn print_password_rule() {
    let password_rule = r#"
    The password must satisfy:
    1. Number of characters must be between 6 and 16.
    2. Must contain:
    - At least one digit (0-9).
    - At least one special character (!@#$%^&*).
    3. Can only contain:
    - Letters (a-z, A-Z).
    - Digits (0-9).
    - Special characters (!@#$%^&*).
    "#;
    println!("{}", password_rule.red());
}

pub fn print_help_msg_by_default() {
    println!("{}", "Commands:".cyan());
    println!("Show help message: {}", HELP_CMD.cyan());
    println!(
        "Signup: {} {}",
        SIGNUP_CMD.cyan(),
        "[username] [email] [password]".cyan()
    );
    println!(
        "Login: {} {}",
        LOGIN_CMD.cyan(),
        "[username] [password]".cyan()
    );
    println!("Exit: {}", EXIT_CMD.cyan());

    print_user_name_rule();
    print_password_rule();
}

pub fn print_help_msg_after_login() {
    println!("{}", "Commands:".cyan());
    println!("Show help message: {}", HELP_CMD.cyan());
    println!("Logout: {}", LOGOUT_CMD.cyan());
    println!("List all the users: {}", LIST_USERS_CMD.cyan());
    println!(
        "Check the status based on username: {} {}",
        CHECK_USER_STATUS_CMD.cyan(),
        "[username]".cyan()
    );
    println!(
        "Create private chat with a user: {} {}",
        PRIVATE_CHAT_CMD.cyan(),
        "[with_user_name]".cyan()
    );
    println!(
        "List all the private chat recipients: {}",
        LIST_RECIPIENTS_CMD.cyan()
    );
    println!(
        "Resume private chat: {} {}",
        RESUME_CHAT_CMD.cyan(),
        "[recipient]".cyan()
    );
    println!(
        "Create chat room with a list of users: {} {}",
        CHAT_ROOM_CMD.cyan(),
        "[group_name] [user1] [user2] [user3]...".cyan()
    );
    println!(
        "Join an existing chat room: {} {}",
        JOIN_CHAT_ROOM_CMD.cyan(),
        "[id]".cyan()
    );
    println!("List existing chat rooms: {}", LIST_CHAT_ROOMS_CMD.cyan());
    println!("Exit: {}", EXIT_CMD.cyan());
}

pub fn print_warning_error_msg(msg: &str) {
    println!("{}", msg.red());
}

pub fn print_msg(msg: &str) {
    println!("{}", msg.green());
}

pub fn print_session_exists_error_msg() {
    print_warning_error_msg("You have already logged in!");
    print_warning_error_msg("Please log out first!");
}

pub fn print_session_not_exist_error_msg() {
    print_warning_error_msg("Please login first!");
}
