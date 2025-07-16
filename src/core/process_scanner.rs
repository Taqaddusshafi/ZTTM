// src/core/process_scanner.rs
use sysinfo::{System, SystemExt, ProcessExt};

pub fn scan_processes() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("🔍 Active Processes:");
    for (pid, process) in system.processes() {
        println!(
            "[PID: {}] {} (Status: {:?})",
            pid,
            process.name(),
            process.status()
        );
    }
}
