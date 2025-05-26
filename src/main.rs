use std::io;

use process::create_process;
mod process;
const COMMAND_PATH: &str = "/Users/cristianaandrei/facultate/uniprocess-os/src/bin";

fn main() {
    loop {
        println!("Enter a command (greet, ls, mkdir, touch, exit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut cmd_args = "";
        if parts.len() > 1 {
            cmd_args = parts[1];
        }

        let cmd = format!("{}/{}.wasm", COMMAND_PATH, parts[0]);
        create_process(cmd.to_string(), &[cmd_args.to_owned()]);
    }
}
