use std::net::Ipv4Addr;
use std::process::Command;
use std::path::Path;

pub fn detect_os(ip: Ipv4Addr) -> String {
    // Try p0f first (passive detection)
    if let Some(os) = try_p0f(&ip) {
        return os;
    }
    
    // Fallback to nmap (active detection)
    if let Some(os) = try_nmap(&ip) {
        return os;
    }
    
    // If all detection methods fail, return unknown
    "OS unknown".to_string()
}

fn try_p0f(ip: &Ipv4Addr) -> Option<String> {
    // Check if p0f is installed
    if !is_command_available("p0f") {
        return None;
    }
    
    // Check if p0f log file exists
    let log_path = if cfg!(target_os = "windows") {
        "C:\\Program Files\\p0f\\p0f.log"
    } else {
        "/var/log/p0f.log"
    };
    
    if !Path::new(log_path).exists() {
        return None;
    }
    
    // Try to run p0f
    let output = Command::new("p0f")
        .args(["-r", log_path])
        .output()
        .ok()?;
    
    let result = String::from_utf8_lossy(&output.stdout);
    
    // Look for the IP in the output
    for line in result.lines() {
        if line.contains(&ip.to_string()) {
            // Extract OS information
            if let Some(os_info) = extract_os_info(line) {
                return Some(format!("{} (via p0f)", os_info));
            }
        }
    }
    
    None
}

fn try_nmap(ip: &Ipv4Addr) -> Option<String> {
    // Check if nmap is installed
    if !is_command_available("nmap") {
        return None;
    }
    
    // Try to run nmap with OS detection
    let output = Command::new("nmap")
        .args(["-O", &ip.to_string()])
        .output()
        .ok()?;
    
    let result = String::from_utf8_lossy(&output.stdout);
    
    // Look for OS information in the output
    for line in result.lines() {
        if line.contains("Running:") || line.contains("OS details:") {
            return Some(format!("{} (via nmap)", line.trim()));
        }
    }
    
    None
}

fn is_command_available(command: &str) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

fn extract_os_info(line: &str) -> Option<String> {
    // Simple extraction - in a real implementation, this would be more sophisticated
    if line.contains("Windows") {
        Some("Windows".to_string())
    } else if line.contains("Linux") {
        Some("Linux".to_string())
    } else if line.contains("macOS") || line.contains("Mac OS") {
        Some("macOS".to_string())
    } else if line.contains("iOS") {
        Some("iOS".to_string())
    } else if line.contains("Android") {
        Some("Android".to_string())
    } else {
        Some(line.to_string())
    }
}