use dhcp_project::server::DhcpServer;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use dhcp_project::database::init::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple logging
    println!("[INFO] DHCP Server starting...");

    // Create SQLite pool
    let manager = SqliteConnectionManager::file("dhcp.db");
    let pool = Pool::new(manager).expect("[ERROR] Failed to create DB pool");

    // Initialize database
    {
        let conn = pool.get().expect("[ERROR] Failed to get DB connection");
        init_db(&conn).expect("[ERROR] Failed to initialize DB schema");
    }

    // Create DHCP server
    let dhcp_server = DhcpServer::new(pool);

    // Start listening on port 67 (default DHCP port)
    let socket = UdpSocket::bind("0.0.0.0:67").await?;
    socket.set_broadcast(true)?;

    println!("[INFO] Listening on 0.0.0.0:67...");

    // Run the server
    dhcp_server.run(socket).await?;
    
    Ok(())
}
