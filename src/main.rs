#[allow(unused_imports)]
use std::io::{self, Write};
use std::ops::Deref;
use std::process::exit;
use std::str::FromStr;

enum Type {
    BuiltIn,
    Unknown
}

enum CommandType {
    Exit(i32),
    Echo {
        value: String,
    },
    Type(Box<Command>),
    Other {
        name: String,
    }
}

struct Command {
    name: String,
    typ: Type,
    command_type: CommandType,
}

impl Command {
    fn execute(&self) {
        match &self.command_type {
            CommandType::Exit(code, ..) => {
                exit(*code);
            },
            CommandType::Echo {value, ..} => {
                println!("{value}");
            },
            CommandType::Type(inner) => {
                match inner.typ {
                    Type::BuiltIn => {
                        println!("{} is a shell builtin", inner.name);
                    },
                    Type::Unknown => {
                        println!("{}: command not found", inner.name);
                    }
                }
            }
            CommandType::Other {name, ..} => {
                println!("{}: command not found", name);
            }
        }
    }
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
                let code = remaining_items.parse::<i32>().unwrap_or(0);
                Ok(
                    Self {
                        name: first_item.into(),
                        command_type: CommandType::Exit(code),
                        typ: Type::BuiltIn,
                    }
                )
            },
            "echo" => {
                Ok(
                    Self {
                        name: first_item.into(),
                        command_type: CommandType::Echo {
                            value: remaining_items,
                        },
                        typ: Type::BuiltIn,
                    }
                )
            },
            "type" => {
                Ok(
                    Self {
                        name: first_item.into(),
                        command_type: CommandType::Type(
                            Box::new(Command::from_str(&remaining_items)?)
                        ),
                        typ: Type::BuiltIn,
                    }
                )
            },
            _ => {
                Ok(
                    Self {
                        name: s.into(),
                        command_type: CommandType::Other {
                            name: s.into(),
                        },
                        typ: Type::Unknown,
                    },
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
        command.execute()
    }
}
