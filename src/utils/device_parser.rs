use serde_json::Value;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct DeviceInfo {
    pub mac_address: String,
    pub ip_address: Ipv4Addr,
    pub device_type: String,
    pub brand: String,
    pub connection_type: String,
    pub data_transferred_bytes: u64,
}

pub fn extract_devices(json_data: &Value) -> Vec<DeviceInfo> {
    let mut devices = Vec::new();

    if let Some(packets) = json_data.as_array() {
        for packet in packets {
            if let Some(layers) = packet.get("_source").and_then(|src| src.get("layers")) {
                if let Some(eth) = layers.get("eth") {
                    if let Some(mac_src) = eth.get("eth.src").and_then(|v| v.as_str()) {
                        if let Some(ip_layer) = layers.get("ip") {
                            if let Some(ip_src) = ip_layer.get("ip.src").and_then(|v| v.as_str()) {
                                if let Ok(ip) = ip_src.parse::<Ipv4Addr>() {
                                    let device = DeviceInfo {
                                        mac_address: mac_src.to_string(),
                                        ip_address: ip,
                                        device_type: "Unknown".to_string(), // à améliorer si besoin
                                        brand: "Unknown".to_string(),       // à améliorer via MAC lookup
                                        connection_type: "Wi-Fi".to_string(), // simplification
                                        data_transferred_bytes: 0,          // à remplir si dispo
                                    };
                                    devices.push(device);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    devices
}
