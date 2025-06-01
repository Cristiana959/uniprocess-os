use std::fs;

fn list_directory(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("Listing files in '{}':", path);
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => {
            println!("Failed to list directory: {}", e);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        list_directory(&args[1].clone());
    } else {
        list_directory(".");
    }
}
