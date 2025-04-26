#[derive(Clone, Debug)]
pub enum DeviceType {
    Phone,
    PC,
    Printer,
    Router,
    IoT,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub mac: String,
    pub manufacturer: String,
    pub device_type: DeviceType,
}

pub fn detect_device(mac: &str) -> DeviceInfo {
    // First convert to uppercase and replace colons
    let mac_uppercase = mac.to_uppercase();
    let mac_clean = mac_uppercase.replace(":", "");
    
    // Then get the prefix
    let prefix = if mac_clean.len() >= 6 {
        &mac_clean[0..6]
    } else {
        "UNKNOWN"
    };
    
    let (manufacturer, device_type) = match prefix {
        // Apple devices
        "001A2B" | "001451" | "001B63" | "001C42" | "001D4F" | "001E52" | "001F5B" | "002241" | "002332" | "002500" => 
            ("Apple", DeviceType::Phone),
        
        // Dell devices
        "001B44" | "001C23" | "001D7D" | "001DE0" | "001E0C" | "001E4F" | "001F6B" | "002248" | "002332" | "002500" => 
            ("Dell", DeviceType::PC),
        
        // Samsung devices
        "3C5AB4" | "00166C" | "001A8A" | "001B98" | "001C62" | "001D7D" | "001E58" | "001F9E" | "002248" | "002332" => 
            ("Samsung", DeviceType::Phone),
        
        // Xiaomi devices
        "A45046" | "28E02C" | "28E14C" | "28E7CF" | "28ED6A" | "28EF01" | "28F076" | "28F10E" | "28F366" | "28F532" => 
            ("Xiaomi", DeviceType::Phone),
        
        // HP printers
        "001451" | "001B78" | "001C23" | "001D7D" | "001E0C" | "001E4F" | "001F6B" | "002248" | "002332" | "002500" => 
            ("HP", DeviceType::Printer),
        
        // Cisco routers
        "00100C" | "00100D" | "00100E" | "00100F" | "001010" | "001011" | "001012" | "001013" | "001014" | "001015" => 
            ("Cisco", DeviceType::Router),
        
        // IoT devices
        "001451" | "001B78" | "001C23" | "001D7D" | "001E0C" | "001E4F" | "001F6B" | "002248" | "002332" | "002500" => 
            ("Generic IoT", DeviceType::IoT),
        
        _ => ("Unknown", DeviceType::Unknown),
    };
    DeviceInfo {
        mac: mac.to_string(),
        manufacturer: manufacturer.to_string(),
        device_type,
    }
}

pub fn get_device_type(mac: &str) -> DeviceType {
    detect_device(mac).device_type
}

pub fn get_manufacturer(mac: &str) -> String {
    detect_device(mac).manufacturer
}