use colored::Colorize;

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
    println!("Show help message: {}", "help".cyan());
    println!("Signup: {}", "signup [username] [email] [password]".cyan());
    println!("Login: {}", "login [username] [password]".cyan());

    print_user_name_rule();
    print_password_rule();
}

pub fn print_help_msg_after_login() {
    println!("{}", "Commands:".cyan());
    println!("Show help message: {}", "help".cyan());
    println!("Logout: {}", "logout".cyan());
    println!("List all active users: {}", "list-users".cyan());
    println!("Check the status based on username: {}", "check [username]".cyan());
    println!("Create private chat with a user: {}", "private-chat [with_user_name]".cyan());
    println!("Resume private chat: []");
    println!("Create chat room with a list of users: {}", "chat-room [group_name] [user1] [user2] [user3]...".cyan());
    println!("Join an existing chat room: []");
    println!("List existing chat rooms: {}", "list-chat-rooms".cyan())
}

pub fn print_warning_error_msg(msg: &str) {
    println!("{}", msg.red());
}

pub fn print_msg(msg: &str) {
    println!("{}", msg.green());
}

pub fn print_session_exists_error_msg () {
    print_warning_error_msg("You have already logged in!");
    print_warning_error_msg("Please log out first!");
}

pub fn print_session_not_exist_error_msg () {
    print_warning_error_msg("Please login first!");
}