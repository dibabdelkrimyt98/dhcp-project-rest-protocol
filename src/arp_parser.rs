use std::net::Ipv4Addr;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub ip: Ipv4Addr,
    pub mac: String,
}

pub fn get_arp_table() -> Option<Vec<ArpEntry>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("arp").arg("-a").output().ok()?
    } else {
        Command::new("arp").arg("-n").output().ok()?
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = vec![];

    for line in stdout.lines() {
        if let Some(entry) = parse_arp_line(line) {
            entries.push(entry);
        }
    }

    Some(entries)
}

fn parse_arp_line(line: &str) -> Option<ArpEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 { return None; }

    if let Ok(ip) = parts[0].parse::<Ipv4Addr>() {
        let mac = parts[1].to_string().replace("-", ":").to_uppercase();
        return Some(ArpEntry { ip, mac });
    }

    None
}
