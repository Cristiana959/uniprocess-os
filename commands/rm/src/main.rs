use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn delete_path(path: &Path, recursive: bool) -> io::Result<()> {
    if path.is_dir() {
        if recursive {
            fs::remove_dir_all(path)?;
            println!("Removed directory: {}", path.display());
        } else {
            eprintln!("{} is a directory. Use -r to remove recursively.", path.display());
        }
    } else if path.is_file() {
        fs::remove_file(path)?;
        println!("Removed file: {}", path.display());
    } else {
        eprintln!("{} does not exist or is not a regular file/directory.", path.display());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "rm" {
        eprintln!("Usage: {} rm [-r] <file_or_directory>", args[0]);
        std::process::exit(1);
    }

    let (recursive, targets) = if args[2] == "-r" {
        (true, &args[3..])
    } else {
        (false, &args[2..])
    };

    if targets.is_empty() {
        eprintln!("Error: No files or directories specified.");
        std::process::exit(1);
    }

    for target in targets {
        let path = Path::new(target);
        if let Err(e) = delete_path(path, recursive) {
            eprintln!("Error removing {}: {}", target, e);
        }
    }
}
