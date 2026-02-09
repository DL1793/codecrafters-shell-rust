#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::process::Command;
mod find_exec;

enum BuiltIns {
    Type,
    Echo,
    Exit
}

impl BuiltIns {
    fn from_command(cmd: &str) -> Option<BuiltIns> {
        match cmd {
            "echo" => Some(BuiltIns::Echo),
            "exit" => Some(BuiltIns::Exit),
            "type" => Some(BuiltIns::Type),
            _ => None,
        }
    }
}

fn main() {
    loop {
        
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        
        let mut parts = command.trim().splitn(2, ' ');

        let cmd_str = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("");

        match BuiltIns::from_command(cmd_str) {
            Some(BuiltIns::Exit) => process::exit(0),
            Some(BuiltIns::Echo) => println!("{}", args),
            Some(BuiltIns::Type) => {
                match BuiltIns::from_command(args) {
                    Some(_) => {
                        println!("{} is a shell builtin", args);
                    }
                    None => {
                        match find_exec::find_executable(args) {
                            Some(path) => println!("{} is {}", args, path),
                            None => println!("{}: not found", args),
                        }
                        
                    }
                }
            }
            None => {
                if let Some(path) = find_exec::find_executable(cmd_str) {
                    let mut child = process::Command::new(cmd_str)
                        .args(args.split_whitespace())
                        .spawn();

                    match child {
                        Ok(mut child) => {
                            let _ = child.wait();
                        },
                        Err(e) => eprintln!("Failed to start command: {}", e),
                    }
                }
                else {
                    println!("{}: not found", cmd_str)
                }
            }
        }

    }
}
