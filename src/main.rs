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

        println!("{}: command not found", command.trim())
    }
}
