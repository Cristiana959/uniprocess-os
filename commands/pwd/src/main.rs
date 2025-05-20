use std::env;

fn main() {
    match env::current_dir() {
        Ok(path) => {
            println!("Current directory: {}", path.display());
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}