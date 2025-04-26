use std::fs::File;
use std::io::Write;
use std::net::Ipv4Addr;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct DeviceRecord {
    pub ip: Ipv4Addr,
    pub mac: String,
    pub manufacturer: String,
    pub device_type: String,
}

pub fn export_to_json(records: &[DeviceRecord], path: &str) -> std::io::Result<()> {
    // Create directory if it doesn't exist
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Convert records to JSON
    let json = serde_json::to_string_pretty(records)?;
    
    // Write to file
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}
    