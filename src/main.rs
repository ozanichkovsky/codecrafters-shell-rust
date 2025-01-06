use std::env;
use std::env::{current_dir, set_current_dir};
use std::ffi::OsStr;
use std::process::Command as CommandRunner;
use std::io::{self, Write};
use std::path::PathBuf;
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
    Pwd,
    Cd {
        path: String,
    },
    Other {
        name: String,
        parameters: Vec<String>
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
            CommandType::Pwd => {
                println!("{}", current_dir().unwrap().display());
            },
            CommandType::Cd {path} => {
                if let Err(err) = set_current_dir(path) {
                    println!("cd: {}: No such file or directory", path);
                }
            }
            CommandType::Type(inner) => {
                match inner.typ {
                    Type::BuiltIn => {
                        println!("{} is a shell builtin", inner.name);
                    },
                    Type::Unknown => {
                        match env::var("PATH") {
                            Ok(paths) => {
                                // Split the PATH into individual paths using `split_paths`
                                let path = find_in_path(&inner.name);
                                match path {
                                    Some(p) => {
                                        println!("{} is {}", &inner.name, p.display());
                                    },
                                    None => {
                                        println!("{}: not found", &inner.name);
                                    }
                                };
                            }
                            Err(e) => {println!("{}: not found", &inner.name);},
                        }
                    }
                }
            }
            CommandType::Other {name, parameters} => {
                let path = find_in_path(name);
                if let Some(p) = path {
                    let mut cmd = CommandRunner::new(p);
                    cmd.arg(parameters.join(" "));
                    let output = cmd.output().unwrap();
                    print!("{}", String::from_utf8(output.stdout).unwrap());
                } else {
                    println!("{}: not found", name);
                }
            }
        }
    }
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = env::var("PATH")
        .ok() // Convert Result to Option
        .and_then(|path_var| {
            // Use iterator to find the first directory containing the file
            env::split_paths(&path_var)
                .find(|path| path.join(name).is_file())
        });
    match path {
        Some(p) => {
            Some(p.join(name))
        },
        None => {
            None
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let first_item = parts.next().unwrap_or(""); // Retrieve the first item or default to an empty string

        let params = parts.map(|i| i.into()).collect::<Vec<String>>();
        // Join remaining items, or handle the case where there are none
        let remaining_items = params.join(" ");
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
            "pwd" => {
                Ok(
                    Self {
                        name: first_item.into(),
                        command_type: CommandType::Pwd,
                        typ: Type::BuiltIn,
                    }
                )
            }
            "cd" => {
                Ok(
                    Self {
                        name: first_item.into(),
                        command_type: CommandType::Cd{
                            path: remaining_items,
                        },
                        typ: Type::BuiltIn,
                    }
                )
            }
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
                        name: first_item.into(),
                        command_type: CommandType::Other {
                            name: first_item.into(),
                            parameters: params,
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
