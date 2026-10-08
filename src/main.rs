use std::io::{self, Write};
use std::str::FromStr;
use codecrafters_shell::commands::{Command};

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if let Ok(command) = Command::from_str(input) {
            command.execute()
        }
    }
}
