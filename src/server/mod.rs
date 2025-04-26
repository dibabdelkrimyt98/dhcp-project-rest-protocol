use tokio::net::UdpSocket;
use std::{net::Ipv4Addr, collections::HashMap, sync::Arc, time::{Duration, SystemTime}};
use tokio::sync::Mutex;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
pub mod ip_pool;
use self::ip_pool::IpPool;

use crate::message::DHCPMessage;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Discover,
    Offer,
    Request,
    Ack,
}

#[derive(Clone)]
pub struct Lease {
    pub mac: String,
    pub expiry: SystemTime,
}

pub struct DhcpServer {
    pub leased_ips: Arc<Mutex<HashMap<Ipv4Addr, Lease>>>,
    pub db_pool: r2d2::Pool<SqliteConnectionManager>,
    pub ip_pool: Arc<Mutex<IpPool>>,
}

impl DhcpServer {
    pub fn new(pool: r2d2::Pool<SqliteConnectionManager>) -> Self {
        let leased = HashMap::new(); // Can load from DB
        DhcpServer {
            leased_ips: Arc::new(Mutex::new(leased)),
            db_pool: pool.clone(),
            ip_pool: Arc::new(Mutex::new(IpPool::new("192.168.1.100", "192.168.1.200"))),
        }
    }

    pub async fn run(&self, socket: UdpSocket) -> std::io::Result<()> {
        let mut buf = [0; 1024];
        println!("[INFO] DHCP Server ready...");

        use tokio::time::timeout;
        match timeout(Duration::from_secs(60), socket.recv_from(&mut buf)).await {
            Ok(Ok((amt, src))) => {
                if let Err(e) = self.handle_packet(buf[..amt].to_vec(), src).await {
                    eprintln!("Error handling packet: {}", e);
                }
            }
            _ => {
                println!("[INFO] No client found in 60s. Server exiting.");
                return Ok(());
            }
        }

        loop {
            let (amt, src) = socket.recv_from(&mut buf).await?;
            if let Err(e) = self.handle_packet(buf[..amt].to_vec(), src).await {
                eprintln!("Error handling packet: {}", e);
            }
        }
    }

    async fn handle_packet(&self, data: Vec<u8>, src: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let msg = String::from_utf8_lossy(&data);
    
        if msg == "DISCOVER" {
            println!("📥 DISCOVER reçu de {}", src);
    
            // Simuler récupération adresse MAC du client (Normalement extraite du paquet DHCP complet)
            // Pour l'exemple, MAC = src IP tronquée
            let ip_addr = match src.ip() {
                std::net::IpAddr::V4(ipv4) => ipv4,
                _ => return Ok(()), // Skip if not IPv4
            };
            
            let simulated_mac = format!("AA:BB:CC:{:02X}:{:02X}:{:02X}", 
                ip_addr.octets()[0],
                ip_addr.octets()[1],
                ip_addr.octets()[2]
            );
    
            // Vérifier si le MAC a déjà une IP enregistrée
            let previous_ip = {
                let conn = self.db_pool.get()?;
                let mut stmt = conn.prepare("SELECT ip FROM leases WHERE mac = ?1")?;
                let ip_iter = stmt.query_map(params![simulated_mac], |row| row.get(0))?;
    
                let mut result = None;
                for ip_result in ip_iter {
                    result = Some(ip_result?);
                }
                result
            };
    
            if let Some(ip_str) = previous_ip {
                println!("🔁 MAC déjà connu, réattribution IP {}", ip_str);
    
                let offer = DHCPMessage {
                    msg_type: "OFFER".to_string(),
                    ip: ip_str,
                };
                let response = serde_json::to_string(&offer)?;
                let socket = UdpSocket::bind("0.0.0.0:0").await?;
                socket.send_to(response.as_bytes(), src).await?;
            } else {
                println!("🆕 Nouveau client, allocation IP aléatoire");
    
                let mut pool = self.ip_pool.lock().await;
                if let Some(ip) = pool.get_random() {
                    let offer = DHCPMessage {
                        msg_type: "OFFER".to_string(),
                        ip: ip.to_string(),
                    };
                    let response = serde_json::to_string(&offer)?;
                    let socket = UdpSocket::bind("0.0.0.0:0").await?;
                    socket.send_to(response.as_bytes(), src).await?;
                    
                    // Sauvegarder dans leases
                    let conn = self.db_pool.get()?;
                    conn.execute(
                        "INSERT OR REPLACE INTO leases (ip, mac, expiry) VALUES (?1, ?2, ?3)",
                        params![ip.to_string(), simulated_mac, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs()],
                    )?;
                } else {
                    println!("❌ Pas d'IP disponible !");
                }
            }
        }
        Ok(())
    }
}
