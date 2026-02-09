#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        
       if command.trim() == "exit" {
        process::exit(0)
       }

        if command.starts_with("echo") {    
            if let Some(args) = command.trim().strip_prefix("echo ") {
                println!("{}", args);
       }
       }
       else {
        println!("{}: command not found", command.trim())
       }
    }
}
