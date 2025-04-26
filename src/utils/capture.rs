use std::process::{Command, Stdio};

pub fn start_capture(filename: &str, capture_time_secs: u64) {
    println!("📡 Capture démarrée sur fichier '{}'", filename);

    let interface = "Wi-Fi"; // Adapter selon ta machine ("Wi-Fi", "Ethernet", "wlan0" sous Linux)

    let _child = Command::new("tshark")
        .args([
            "-i", interface,
            "-a", &format!("duration:{}", capture_time_secs),
            "-w", filename,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("[ERREUR] Impossible de lancer tshark");

    println!("⌛ Capture en cours...");
}
