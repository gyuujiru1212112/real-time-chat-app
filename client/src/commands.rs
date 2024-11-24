use validator::ValidateEmail;
use regex::Regex;


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
        println!("Error: Invalid username.");
        let rule = r#"
The username must satisfy:
1. Number of characters must be between 5 and 20.
2. Must start with a letter.
3. Only alphanumeric characters (a-z, A-Z, 0-9), underscores (_) and dots (.) are allowed.
4. No consecutive underscores or dots (e.g., __ or ..).
5. No underscore or dot at the end.
"#;
        println!("{}", rule);
    }
    res
}

pub fn is_valid_password(password: &str) -> bool
{
    let rule = r#"
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
    // 6 - 16
    if password.len() < 6 || password.len() > 16 {
        println!("Error: Invalid password.");
        println!("{}", rule);
        return false;
    }

    // at least one digit
    let has_digit = Regex::new(r"[0-9]").unwrap();
    if !has_digit.is_match(password) {
        println!("Error: Invalid password.");
        println!("{}", rule);
        return false;
    }

    // at least one special character
    let has_special = Regex::new(r"[!@#$%^&*]").unwrap();
    if !has_special.is_match(password) {
        println!("Error: Invalid password.");
        println!("{}", rule);
        return false;
    }

    // all characters are valid
    let valid_chars = Regex::new(r"^[a-zA-Z0-9!@#$%^&*]+$").unwrap();
    if !valid_chars.is_match(password) {
        println!("Error: Invalid password.");
        println!("{}", rule);
        return false;
    }

    true
}

// pub async fn signup(client: &Client, username: String, email: String, password: String) -> Result<(), Box<dyn StdError>>
// {
//     let url = "http://localhost:8000/chatapp/user/signup"; // signup endpoint

//     // Prepare the signup data
//     let signup_info = SignupInfo { username, email, password };

//     // Send the POST request
//     let response = client
//         .post(url)
//         .json(&signup_info)
//         .send()
//         .await?;

//     // Check if the response was successful
//     if response.status().is_success() {
//         println!("Signup successfully!");
//     } else {
//         let error_message = response.text().await?;
//         println!("Error: failed to signup user: {}.", error_message);
//     }

//     Ok(())
// }

// pub async fn login(client: &Client, username: String, password: String) -> Result<(), Box<dyn StdError>>
// {
//     let url = "http://localhost:8000/chatapp/user/login";
//     let login_info = UserLogin { username, password };

//     // Send the POST request
//     let response = client
//         .post(url)
//         .json(&login_info)
//         .send()
//         .await?;

//     // Check if the response was successful
//     if response.status().is_success() {
//         println!("login successfully!");
//         let json : Value = response.json().await.expect("Failed to parse JSON");

//         // get the session id
//         if let Some(session_id) = json.get("session_id") {
//             println!("session_id: {}", session_id);
//         }


//     } else {
//         let error_message = response.text().await?;
//         println!("Error: failed to login user: {}.", error_message);
//     }

//     Ok(())
// }

// async fn logout(client: &Client) -> Result<(), Box<dyn StdError>>
// {
//     // todo session id
//     Ok(())
// }