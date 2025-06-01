use std::fs::File;
use std::io::{self, Write};

pub fn create_file(path: &str) {
    // File path relative to the pre-opened directory (current directory)
    let mut file = File::create(path).unwrap();
    writeln!(file, "Hello from WASM!");
}

fn main() -> io::Result<()> {
    // Call the function that creates the file
    let args: Vec<String> = std::env::args().collect();
    if args[1] != "" {
        create_file(&args[1].clone());
    } else {
        println!("Please provide the directory path.")
    }
    println!("File created successfully!");
    Ok(())
}
