use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::time::{Instant, Duration};

pub struct SecurityManager {
    blacklist: HashSet<String>, // MAC Address
    request_times: HashMap<String, Instant>,
    flood_timeout: Duration,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            blacklist: HashSet::new(),
            request_times: HashMap::new(),
            flood_timeout: Duration::from_secs(5), // 5s entre 2 demandes
        }
    }

    pub fn is_blacklisted(&self, mac: &str) -> bool {
        self.blacklist.contains(mac)
    }

    pub fn add_to_blacklist(&mut self, mac: &str) {
        self.blacklist.insert(mac.to_string());
    }

    pub fn allow_request(&mut self, mac: &str) -> bool {
        let now = Instant::now();
        if let Some(last) = self.request_times.get(mac) {
            if now.duration_since(*last) < self.flood_timeout {
                println!("🚨 Tentative de flood détectée pour {}", mac);
                return false;
            }
        }
        self.request_times.insert(mac.to_string(), now);
        true
    }
}

pub fn is_valid_mac(mac: &str) -> bool {
    let mac_clean = mac.to_uppercase().replace(":", "");
    mac_clean.len() == 12 && mac_clean.chars().all(|c| c.is_ascii_hexdigit())
}
