use std::fs;
use std::io;

fn mkdir(path: String) {
    fs::create_dir(path.as_str()).unwrap();
    println!("Directory '{}' created successfully.", path);
  
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("Args is {:?}", args);
    if args[2] != "" {
        mkdir(args[2].clone());
    } else {
        println!("Please provide the directory path.")
    }
    Ok(())
}