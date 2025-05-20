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
    list_directory(".");
}
