use std::{
    collections::HashMap,
    io,
    sync::{Arc, Mutex},
};

use process::{ProcessTable, create_process, process_manager::print_process_table};
mod process;
const COMMAND_PATH: &str = "/Users/cristianaandrei/facultate/uniprocess-os/src/bin";

fn main() {
    let mut pid = 1;
    let process_table: ProcessTable = Arc::new(Mutex::new(HashMap::new()));
    loop {
        println!("Enter a command (greet, ls, mkdir, touch, exit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut cmd_args = Vec::new();
        if parts.len() > 1 {
            for arg in parts.clone() {
                cmd_args.push(arg.to_string());
            }
        }
        // if parts[0] == "ps" {
        //     print_process_table(&process_table);
        // }

        let cmd = format!("{}/{}.wasm", COMMAND_PATH, parts[0]);
        println!("Executing command {:?}", cmd);

        create_process(
            pid,
            cmd.to_string(),
            cmd_args.as_slice(),
            Arc::clone(&process_table),
        );
        pid += 1;
    }
}
