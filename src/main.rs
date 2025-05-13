use std::{io, path::Path};

use process::create_process;
mod process;
enum Command {
    Greet,
    Ls,
    Exit,
    Unknown,
}

impl Command {
    fn from_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "greet" => Command::Greet,
            "ls" => Command::Ls,
            "exit" => Command::Exit,
            _ => Command::Unknown,
        }
    }
}

fn main() {
    loop {
        println!("Enter a command (greet, ls, exit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let command = Command::from_input(&input);

        match command {
            Command::Greet => {
                println!("Hello there!");
            }
            Command::Ls => {
                create_process(
                    "/Users/cristianaandrei/facultate/uniprocess-os/src/bin/ls.wasm".to_string(),
                );
            }
            Command::Exit => {
                println!("Exiting program.");
                break;
            }
            Command::Unknown => {
                println!("Unknown command.");
            }
        }
    }
}
