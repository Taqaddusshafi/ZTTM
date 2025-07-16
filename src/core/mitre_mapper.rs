use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn map_to_mitre(indicators: &[&str]) {
    let mut mitre_map = HashMap::new();
    mitre_map.insert("powershell", ("T1059.001", "PowerShell Execution"));
    mitre_map.insert("mimikatz", ("T1003", "Credential Dumping"));
    mitre_map.insert("autorun", ("T1547", "Boot or Logon Autostart Execution"));
    mitre_map.insert("schtasks", ("T1053.005", "Scheduled Task - Windows"));
    mitre_map.insert("runonce", ("T1547.001", "Registry Run Keys"));

    println!("🧠 MITRE ATT&CK Mapping:");

    for indicator in indicators {
        let indicator_lc = indicator.to_lowercase();

        for (pattern, (ttp, desc)) in &mitre_map {
            if indicator_lc.contains(pattern) {
                println!("⚔️  Match: [{}] {} => {}", ttp, desc, indicator);

                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("alerts.log")
                {
                    let _ = writeln!(file, "⚔️ [{}] {} => {}", ttp, desc, indicator);
                }
            }
        }
    }

    println!("✅ MITRE mapping complete.");
}
