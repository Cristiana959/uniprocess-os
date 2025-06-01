use super::ProcessTable;

pub fn print_process_table(process_table: &ProcessTable) {
    let table = process_table.lock().unwrap();
    println!(
        "{:<5} {:<15} {:<10} {}",
        "PID", "Name", "Status", "Uptime(s)"
    );

    for info in table.values() {
        println!(
            "{:<5} {:<15} {:<10} {}",
            info.pid, info.name, info.status, info.started_at
        );
    }
}
