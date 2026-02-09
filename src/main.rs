#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

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
                        println!("{}: not found", args)
                    }
                }
            }
            None => {
                println!("{}: not found", cmd_str)
            }
        }

    }
}
