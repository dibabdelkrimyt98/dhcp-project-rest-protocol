use serde_json::Value;
use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub mac_address: String,
    #[serde(with = "ipv4_addr_serde")]
    pub ip_address: Ipv4Addr,
    pub device_type: String,
    pub brand: String,
    pub connection_type: String,
    pub data_transferred_bytes: u64,
}

pub mod ipv4_addr_serde {
    use std::net::Ipv4Addr;
    use serde::{Serializer, Deserializer, Deserialize};

    pub fn serialize<S>(ip: &Ipv4Addr, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&ip.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ipv4Addr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
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
