use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Host {
    pub ip: Ipv4Addr,
}

pub fn scan_subnet(subnet: &str) -> Vec<Host> {
    let mut hosts = vec![];

    let base: Vec<&str> = subnet.split('.').collect();
    if base.len() != 4 { return hosts; }

    let prefix = format!("{}.{}.{}.", base[0], base[1], base[2]);
    for i in 1..255 {
        let ip = format!("{}{}", prefix, i);
        let ip_clone = ip.clone();
        thread::spawn(move || {
            let _ = Command::new("ping")
                .args(["-c", "1", "-W", "1", &ip_clone])
                .output();
        });

        // Add to result list
        if let Ok(parsed) = ip.parse::<Ipv4Addr>() {
            hosts.push(Host { ip: parsed });
        }
        thread::sleep(Duration::from_millis(5)); // léger délai pour éviter surcharge CPU
    }

    hosts
}
