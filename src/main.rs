#[allow(unused_imports)]
mod locator;
mod builtins;

use std::io::{self, Write};
use std::process;
use std::env::set_current_dir;
use std::os::unix::process::CommandExt;
use crate::builtins::BuiltIns;





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
            Some(BuiltIns::Cd) => {
                let new_dir = args.trim();

                let path = if new_dir == "~" {
                    std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
                }
                else {
                    new_dir.to_string()
                };

                if let Err(e) = std::env::set_current_dir(&path) {
                    eprintln!("cd: {}: No such file or directory", path);
                }
            }
            Some(BuiltIns::Pwd) => {
                match std::env::current_dir() {
                    Ok(path) => {
                        println!("{}", path.display());
                    }
                    Err(e) => {
                        eprintln!("Error retrieving current directory: {}", e);
                    }
                }
            }
            Some(BuiltIns::Type) => {
                match BuiltIns::from_command(args) {
                    Some(_) => {
                        println!("{} is a shell builtin", args);
                    }
                    None => {
                        match locator::find_executable(args) {
                            Some(path) => println!("{} is {}", args, path),
                            None => println!("{}: not found", args),
                        }    
                    }
                }
            }
            None => {
                if let Some(path) = locator::find_executable(cmd_str) {

                    let mut child = process::Command::new(path)
                        .arg0(cmd_str)
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
