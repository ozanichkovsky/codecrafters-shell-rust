use std::env;
use std::env::{current_dir, set_current_dir};
use std::process::Command as CommandRunner;
use std::process::exit;
use std::os::unix::process::CommandExt;
use std::str::FromStr;
use crate::parser::tokenize;
use crate::path::{find_in_path, get_path};

enum Type {
    BuiltIn,
    Unknown
}

enum CommandType {
    Exit(i32),
    Echo {
        value: String,
    },
    Type {
        name: String,
    },
    Pwd,
    Cd {
        path: String,
    },
    Other {
        name: String,
        parameters: Vec<String>
    }
}

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn type_of(name: &str) -> Type {
    if BUILTINS.contains(&name) {
        Type::BuiltIn
    } else {
        Type::Unknown
    }
}

pub struct Command {
    name: String,
    typ: Type,
    command_type: CommandType,
}

impl Command {
    pub fn execute(&self) {
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
                if let Err(_err) = set_current_dir(get_path(&path)) {
                    println!("cd: {}: No such file or directory", &path);
                }
            }
            CommandType::Type {name} => {
                match type_of(name) {
                    Type::BuiltIn => {
                        println!("{} is a shell builtin", name);
                    },
                    Type::Unknown => {
                        match env::var("PATH") {
                            Ok(_paths) => {
                                // Split the PATH into individual paths using `split_paths`
                                let path = find_in_path(name);
                                match path {
                                    Some(p) => {
                                        println!("{} is {}", name, p.display());
                                    },
                                    None => {
                                        println!("{}: not found", name);
                                    }
                                };
                            }
                            _ => {println!("{}: not found", name);},
                        }
                    }
                }
            }
            CommandType::Other {name, parameters} => {
                let path = find_in_path(name);
                if let Some(p) = path {
                    let mut cmd = CommandRunner::new(p);
                    cmd.arg0(name).args(parameters);
                    let output = cmd.output().unwrap();
                    print!("{}", String::from_utf8(output.stdout).unwrap());
                } else {
                    println!("{}: not found", name);
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(s);
        let mut parts = tokens.iter();
        let first_item_opt = parts.next(); // Retrieve the first item or default to an empty string
        let first_item: &str;
        if let Some(s) = first_item_opt {
            first_item = s;
        } else {
            return Err("invalid command".into());
        }

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
                        command_type: CommandType::Type {
                            name: params.first().cloned().unwrap_or_default(),
                        },
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