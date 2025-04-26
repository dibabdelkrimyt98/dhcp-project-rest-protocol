use std::{
    collections::{HashMap, HashSet},
    net::{Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
    fs,
};

// Import only what's available in dhcp4r
use dhcp4r::packet::Packet;
use log::{error, info, warn};
use serde::Deserialize;
use tokio::net::UdpSocket;
use tokio::task;

// Define MacAddress as a type alias for [u8; 6]
type MacAddress = [u8; 6];

#[derive(Debug, Clone)]
struct DhcpServer {
    subnets: HashMap<String, SubnetConfig>,
    leased_ips: Arc<Mutex<HashMap<Ipv4Addr, Lease>>>,
    offered_ips: Arc<Mutex<HashMap<u32, OfferedIp>>>,
    reservations: HashMap<String, Ipv4Addr>,
    blacklist: HashSet<MacAddress>,
}

#[derive(Debug, Clone, Deserialize)]
struct SubnetConfig {
    pool: IpPool,
    gateway: Ipv4Addr,
    dns_servers: Vec<Ipv4Addr>,
    subnet_mask: Ipv4Addr,
}

#[derive(Debug, Clone)]
struct Lease {
    mac: MacAddress,
    expiry: SystemTime,
}

#[derive(Debug, Clone)]
struct OfferedIp {
    mac: MacAddress,
    ip: Ipv4Addr,
    expiry: SystemTime,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    subnets: HashMap<String, SubnetConfig>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Create a default server instead of loading from config
    let server = Arc::new(DhcpServer::new_default());
    let socket = UdpSocket::bind("0.0.0.0:6767").await?;
    let mut buf = [0u8; 576];

    let server_clone = server.clone();
    task::spawn(async move {
        server_clone.reclaim_expired_leases().await;
    });

    println!("DHCP Server listening on 0.0.0.0:6767");

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let server = server.clone();
        let data = buf[..len].to_vec();

        task::spawn(async move {
            if let Err(e) = server.handle_packet(data, addr).await {
                error!("Error handling packet: {}", e);
            }
        });
    }
}

// IMPLEMENTATION
impl DhcpServer {
    fn new_default() -> Self {
        // Create a default subnet configuration
        let mut subnets = HashMap::new();
        subnets.insert(
            "192.168.1.0/24".to_string(),
            SubnetConfig {
                pool: IpPool::new(
                    Ipv4Addr::new(192, 168, 1, 100),
                    Ipv4Addr::new(192, 168, 1, 200),
                ),
                gateway: Ipv4Addr::new(192, 168, 1, 1),
                dns_servers: vec![Ipv4Addr::new(8, 8, 8, 8)],
                subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
            },
        );

        DhcpServer {
            subnets,
            leased_ips: Arc::new(Mutex::new(HashMap::new())),
            offered_ips: Arc::new(Mutex::new(HashMap::new())),
            reservations: HashMap::new(),
            blacklist: HashSet::new(),
        }
    }

    fn new_from_config(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&contents)?;

        Ok(DhcpServer {
            subnets: config.subnets,
            leased_ips: Arc::new(Mutex::new(HashMap::new())),
            offered_ips: Arc::new(Mutex::new(HashMap::new())),
            reservations: HashMap::new(),
            blacklist: HashSet::new(),
        })
    }

    async fn handle_packet(&self, data: Vec<u8>, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just print the received data
        println!("Received packet from {}: {:?}", addr, data);
        
        // In a real implementation, we would parse the DHCP packet here
        // and handle different message types
        
        Ok(())
    }

    async fn reclaim_expired_leases(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            let now = SystemTime::now();
            let mut leased_ips = self.leased_ips.lock().unwrap();
            leased_ips.retain(|_, lease| lease.expiry > now);
        }
    }
}

// IP POOL RANGE STRUCTURE
#[derive(Debug, Clone, Deserialize)]
struct IpPool {
    start: Ipv4Addr,
    end: Ipv4Addr,
}

impl IpPool {
    fn new(start: Ipv4Addr, end: Ipv4Addr) -> Self {
        Self { start, end }
    }

    fn range(&self) -> Vec<Ipv4Addr> {
        let start = u32::from_be_bytes(self.start.octets());
        let end = u32::from_be_bytes(self.end.octets());
        (start..=end).map(|val| Ipv4Addr::from(val.to_be_bytes())).collect()
    }
}

// Helper functions for IP address conversion
fn ip_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

fn u32_to_ip(val: u32) -> Ipv4Addr {
    Ipv4Addr::from(val.to_be_bytes())
}
