use std::io;

use process::create_process;
mod process;
enum Command {
    Greet,
    Ls,
    Pwd,
    Mkdir,
    Touch,
    Exit,
    Unknown,
}

impl Command {
    fn from_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "greet" => Command::Greet,
            "ls" => Command::Ls,
            "pwd" => Command::Pwd,
            "mkdir" => Command::Mkdir,
            "touch" => Command::Touch,
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
            Command::Pwd => {
                create_process(
                    "/Users/cristianaandrei/facultate/uniprocess-os/src/bin/pwd.wasm".to_string(),
                );
            }
            Command::Mkdir => {
                create_process(
                    "/Users/cristianaandrei/facultate/uniprocess-os/src/bin/mkdir.wasm".to_string(),
                );
            }
            Command::Touch => {
                create_process(
                    "/Users/cristianaandrei/facultate/uniprocess-os/src/bin/touch.wasm".to_string(),
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
