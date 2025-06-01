use std::env;
use serde::Deserialize;
use chrono::{DateTime, Local};


#[derive(Deserialize)]
struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub started_at: DateTime<Local>,
    pub status: String,
}

fn main() {
    let list = env::var("PROCESS_TABLE").unwrap_or_else(|_| "[]".into());
    let processes: Vec<ProcessInfo> = serde_json::from_str(&list).unwrap_or_default();
    
    let mut uptime;
    println!("{:<5} {:<10} {:<20}", "PID", "NAME", "UPTIME");
    for p in processes {
        uptime = (Local::now() - p.started_at).as_seconds_f32();
        println!("{:<5} {:<10} {:<20}", p.pid, p.name, uptime);
    }
}
