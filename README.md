# ZTTM - Zero-Touch Threat Mapper

ZTTM (Zero-Touch Threat Mapper) is a lightweight, Rust-based cybersecurity utility designed to scan a Windows system for common indicators of compromise (IOCs), suspicious processes, unauthorized startup entries, and known MITRE ATT&CK patterns — all through a simple command-line interface.

## 🧩 Features
- **🧠 MITRE ATT&CK Mapping**: Maps known indicators (like `powershell`, `mimikatz`, `autorun.inf`) to MITRE TTPs for faster threat understanding.
- **🚀 Startup Program Detector**: Scans registry and startup folders for auto-run entries that could indicate persistence mechanisms.
- **🗂️ Executable File Scanner**: Recursively scans directories for suspicious `.exe` files.
- **🧪 Process Scanner**: Checks currently running processes.
- **🌐 Network Sniffer (Planned)**: Identifies traffic on common suspicious ports like `23`, `4444`, etc.

## 🔧 Usage
Run the program with:
```bash
cargo run
```

You'll see a prompt like this:
```
ZTTM: Zero-Touch Threat Mapper
==================================
Please choose an option:
1️⃣  Scan Running Processes
2️⃣  Scan Files for Executables
3️⃣  Detect Startup Programs
4️⃣  Run Network Sniffer
5️⃣  MITRE ATT&CK Mapping
0️⃣  Exit
Enter your choice:
```

### 🧠 MITRE Mapping Example Output
```
⚔️  Match: [T1059.001] PowerShell Execution => powershell.exe
⚔️  Match: [T1003] Credential Dumping => mimikatz
⚔️  Match: [T1547] Boot or Logon Autostart Execution => autorun.inf
```

## 📂 Project Structure
```
src/
├── core/
│   ├── file_scanner.rs
│   ├── process_scanner.rs
│   ├── startup_detector.rs
│   └── mitre_mapper.rs
├── network_sniffer.rs
└── main.rs
```

## 🚀 Build & Run
1. Ensure [Rust](https://www.rust-lang.org/) is installed on your system.
2. Build in release mode for an optimized executable:
   ```bash
   cargo build --release
   ```
3. Run the compiled binary:
   ```bash
   target/release/zttm.exe
   ```

## 📜 License
MIT License © Taqaddus Shafi

## 🛡️ Disclaimer
This tool is intended for educational and authorized research use only. Do not use it on systems without proper permission.

## ✅ Happy Hunting!
