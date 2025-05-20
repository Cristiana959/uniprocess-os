use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let dir = "new_directory";

    // Create a new directory
    fs::create_dir(dir)?;

    println!("Directory '{}' created successfully.", dir);
    Ok(())
}