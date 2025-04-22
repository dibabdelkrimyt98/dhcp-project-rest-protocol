use tokio::net::UdpSocket;
use serde::{Deserialize, Serialize};
use rusqlite::params;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::error::Error;
use r2d2_sqlite::SqliteConnectionManager;
use crate::message::DHCPMessage;
use std::time::{SystemTime, Duration};
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Debug, Serialize, Deserialize)]
enum MessageType {
    Discover,
    Offer,
    Request,
    Ack,
}

struct Lease {
    mac: String,
    expiry: SystemTime,
}

pub struct DhcpServer {
    subnets: Vec<String>,
    leased_ips: Arc<Mutex<HashMap<Ipv4Addr, Lease>>>,
    db_pool: r2d2::Pool<SqliteConnectionManager>,
}

impl DhcpServer {
    pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
        let leased = Self::load_existing_leases(&pool);

        DhcpServer {
            subnets: vec!["192.168.1.0/24".to_string()],
            leased_ips: Arc::new(Mutex::new(leased)),
            db_pool: pool,
        }
    }

    pub async fn run(&self, socket: UdpSocket) -> std::io::Result<()> {
        let mut buf = [0; 1024];
        
        loop {
            let (amt, src) = socket.recv_from(&mut buf).await?;
            let data = buf[..amt].to_vec();
            
            if let Err(e) = self.handle_packet(data, src).await {
                eprintln!("Error handling packet: {}", e);
            }
        }
    }

    async fn handle_packet(&self, data: Vec<u8>, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        // For now, just print the received data
        println!("Received packet from {}: {:?}", addr, data);
        
        // In a real implementation, we would parse the DHCP packet here
        // and handle different message types
        
        Ok(())
    }

    fn load_existing_leases(pool: &r2d2::Pool<SqliteConnectionManager>) -> HashMap<Ipv4Addr, Lease> {
        let conn = pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT ip, mac, expiry FROM leases").unwrap();

        let leases = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?.parse::<Ipv4Addr>().unwrap(),
                    Lease {
                        mac: row.get::<_, String>(1)?,
                        expiry: SystemTime::UNIX_EPOCH + Duration::from_secs(row.get::<_, u64>(2)?),
                    },
                ))
            })
            .unwrap();

        let mut lease_map = HashMap::new();
        for lease in leases {
            let (ip, lease) = lease.unwrap();
            lease_map.insert(ip, lease);
        }

        lease_map
    }

    fn save_lease(&self, ip: Ipv4Addr, mac: String, expiry: SystemTime) -> Result<(), Box<dyn Error>> {
        let conn = self.db_pool.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO leases (ip, mac, expiry) VALUES (?1, ?2, ?3)",
            params![
                ip.to_string(),
                mac,
                expiry.duration_since(SystemTime::UNIX_EPOCH)?.as_secs()
            ],
        )?;
        Ok(())
    }

    fn cleanup_expired_leases(&self) -> Result<(), Box<dyn Error>> {
        let now = SystemTime::now();
        let conn = self.db_pool.get()?;
        conn.execute(
            "DELETE FROM leases WHERE expiry < ?1",
            params![now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs()],
        )?;
        Ok(())
    }
}

#[tokio::main]
pub async fn start_server() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("dhcp.db");
    let pool = r2d2::Pool::new(manager).expect("Failed to create DB pool");
    
    // Initialize database
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS leases (
                ip TEXT PRIMARY KEY,
                mac TEXT NOT NULL,
                expiry INTEGER NOT NULL
            )",
            [],
        ).unwrap();
    }

    let _server = DhcpServer::new(pool);
    let socket = UdpSocket::bind("0.0.0.0:67").await?;
    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).await?;
        let msg = std::str::from_utf8(&buf[..amt]).unwrap_or("");

        if msg == "DISCOVER" {
            println!("Received DISCOVER from {}", src);
            let offer = DHCPMessage {
                msg_type: "OFFER".to_string(),
                ip: "192.168.1.100".to_string(),
            };
            let response = serde_json::to_string(&offer)?;
            socket.send_to(response.as_bytes(), src).await?;
        }
    }
}
