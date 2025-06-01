// pwd.rs
use std::env;

fn main() {
    let pwd = env::var("PWD").unwrap_or_else(|_| ".".to_string());
    println!("{}", pwd);
}
