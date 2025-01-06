#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::str::FromStr;

enum Command {
    Exit(i32),
    Echo(String),
    Other(String)
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let first_item = parts.next().unwrap_or(""); // Retrieve the first item or default to an empty string

        // Join remaining items, or handle the case where there are none
        let remaining_items = parts.collect::<Vec<_>>().join(" ");
        match first_item {
            "exit" => {
                Ok(
                    Self::Exit(0)
                )
            },
            "echo" => {
                Ok(
                    Self::Echo(remaining_items)
                )
            },
            _ => {
                Ok(
                    Self::Other(s.into())
                )
            }
        }
    }
}

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
        let command = Command::from_str(input).unwrap();
        match command {
            Command::Exit(code) => {exit(code);},
            Command::Echo(val) => {
                println!("{}", val);
            }
            Command::Other(name) => {
                println!("{}: command not found", name);
            }
        }
    }
}
