use crate::network_scanner::scan_subnet;
use crate::arp_parser::get_arp_table;
use crate::utils::mac_lookup::{detect_device, DeviceInfo};
use crate::utils::os_detector::detect_os;
use crate::exporter::{DeviceRecord, export_to_json};
use std::time::Instant;

pub fn full_scan_and_export(subnet: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Starting network scan of subnet: {}", subnet);
    let start_time = Instant::now();
    
    // Step 1: Scan the subnet for live hosts
    println!("📡 Scanning subnet for live hosts...");
    let live_hosts = scan_subnet(subnet);
    println!("✅ Found {} live hosts", live_hosts.len());
    
    // Step 2: Get the ARP table
    println!("🔍 Retrieving ARP table...");
    let arp = get_arp_table().unwrap_or_default();
    println!("✅ ARP table retrieved with {} entries", arp.len());
    
    let mut results = vec![];
    
    // Step 3: Process each live host
    println!("🔍 Processing host information...");
    for host in live_hosts {
        if let Some(entry) = arp.iter().find(|e| e.ip == host.ip) {
            // Get device information
            let dev: DeviceInfo = detect_device(&entry.mac);
            
            // Get OS information
            let os = detect_os(host.ip);
            
            // Create device record
            let manufacturer = dev.manufacturer.clone();
            results.push(DeviceRecord {
                ip: host.ip,
                mac: entry.mac.clone(),
                manufacturer: dev.manufacturer,
                device_type: format!("{:?}", dev.device_type),
            });
            
            println!("✅ Processed host: {} ({}) - {} - {:?}", 
                     host.ip, entry.mac, manufacturer, dev.device_type);
        } else {
            println!("⚠️ No MAC address found for host: {}", host.ip);
        }
    }
    
    // Step 4: Export results to JSON
    println!("📝 Exporting results to JSON...");
    export_to_json(&results, output)?;
    
    let duration = start_time.elapsed();
    println!("✅ Export completed in {:.2} seconds: {}", duration.as_secs_f64(), output);
    println!("📊 Total devices found: {}", results.len());
    
    Ok(())
}
