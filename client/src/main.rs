use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the real time chat app!");
    println!("Type 'help' to see available commands.");
    // todo launch the server

    // get the command
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "help" => {
                // todo help message
            }
            command if input.starts_with("signup") => {

            }
            command if input.starts_with("login") => {

            }
            command if input.starts_with("logout") => {

            }
            
            "q" => {
                // exit the app
                println!("App is shutting down...");
                break;
            }
            _ => {
                println!("Unknown command. Type 'help' to see available commands.")
            }
        }
    }
    Ok(())
}