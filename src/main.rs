mod core;

use core::process_scanner::scan_processes;
use core::file_scanner::scan_directory_for_binaries;
use core::startup_detector::detect_startup_programs;
use core::mitre_mapper::map_to_mitre;
mod network_sniffer;

use std::io::{self, Write};

fn main() {
    loop {
        println!("\n🔐 ZTTM: Zero-Touch Threat Mapper");
        println!("==================================");
        println!("Please choose an option:");
        println!("1️⃣  Scan Running Processes");
        println!("2️⃣  Scan Files for Executables");
        println!("3️⃣  Detect Startup Programs");
        println!("4️⃣  Run Network Sniffer");
        println!("5️⃣  MITRE ATT&CK Mapping");
        println!("0️⃣  Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // flush to show prompt

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "1" => {
                println!("🔎 Scanning running processes...");
                scan_processes();
            }
            "2" => {
                println!("📁 Scanning files for executables...");
                scan_directory_for_binaries("C:\\Users");
            }
            "3" => {
                println!("🚀 Detecting Startup Programs...");
                detect_startup_programs();
            }
            "4" => {
                println!("🌐 Running Network Sniffer...");
                network_sniffer::run_network_sniffer();
            }
            "5" => {
                println!("🧠 MITRE ATT&CK Mapping...");
                let indicators = [
                    "powershell.exe",
                    "mimikatz",
                    "autorun.inf",
                    "schtasks /create",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\\runonce"
                ];
                map_to_mitre(&indicators);
            }
            "0" => {
                println!("👋 Exiting ZTTM. Stay safe!");
                break;
            }
            _ => println!("❌ Invalid choice, please try again."),
        }

        println!("🔚 ZTTM Scan Complete.");
    }
}
