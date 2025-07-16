use winreg::enums::*;
use winreg::RegKey;
use std::path::Path;
use std::fs;
use dirs;

pub fn detect_startup_programs() {
    println!("🚀 Startup Program Detector:");

    // Read registry startup entries
    println!("📑 Registry Startup Entries:");

    // HKLM\Software\Microsoft\Windows\CurrentVersion\Run
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_READ);
    if let Ok(key) = hklm {
        for (name, value) in key.enum_values().flatten() {
            let value_str = decode_utf16_le(&value.bytes);
            println!("🧬 [HKLM] {} = {}", name, value_str);
        }
    }

    // HKCU\Software\Microsoft\Windows\CurrentVersion\Run
    let hkcu = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_READ);
    if let Ok(key) = hkcu {
        for (name, value) in key.enum_values().flatten() {
            let value_str = decode_utf16_le(&value.bytes);
            println!("👤 [HKCU] {} = {}", name, value_str);
        }
    }

    // Scan startup folders
    println!("📁 Startup Folder Entries:");

    if let Some(appdata) = dirs::data_dir() {
        let user_startup = appdata.join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
        scan_startup_folder(&user_startup);
    }

    let common_startup = Path::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup");
    scan_startup_folder(common_startup);

    println!("✅ Startup scan complete.");
}

fn scan_startup_folder(path: &Path) {
    println!("📂 Scanning: {}", path.display());
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            println!("📌 Found: {}", file_path.display());
        }
    }
}

// Helper function to decode UTF-16 LE bytes into a Rust String
fn decode_utf16_le(bytes: &[u8]) -> String {
    let utf16: Vec<u16> = bytes
        .chunks(2)
        .filter_map(|b| {
            if b.len() == 2 {
                Some(u16::from_le_bytes([b[0], b[1]]))
            } else {
                None
            }
        })
        .collect();
    String::from_utf16_lossy(&utf16)
}
