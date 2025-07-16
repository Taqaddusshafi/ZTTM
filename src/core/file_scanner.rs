use std::fs;
use std::path::Path;

pub fn scan_directory_for_binaries(path: &str) {
    println!("🗂️ Scanning directory: {}", path);
    walk_directory(Path::new(path));
}

fn walk_directory(path: &Path) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                walk_directory(&path); // recursion
            } else if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ["exe", "dll", "bat", "ps1", "cmd"].contains(&ext.as_str()) {
                    println!("⚠️ Found binary file: {}", path.display());
                }
            }
        }
    }
}
