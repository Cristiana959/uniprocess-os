use std::fs::File;
use std::io::{self, Write};

pub fn create_file() -> io::Result<()> {
    // File path relative to the pre-opened directory (current directory)
    let path = "output.txt"; 
    let mut file = File::create(path)?;
    writeln!(file, "Hello from WASM!")?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Call the function that creates the file
    create_file()?;
    println!("File created successfully!");
    Ok(())
}
