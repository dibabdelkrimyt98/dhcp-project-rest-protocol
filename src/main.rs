mod client;
mod server;
mod message;
mod config;
mod database;
mod api;
mod utils;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::net::UdpSocket;
use dhcp_project::server::DhcpServer;
use dhcp_project::database::init::init_db;
use std::sync::Arc;
use tokio::sync::Mutex;
use config::Config;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args();

    if std::env::args().collect::<Vec<String>>().contains(&"--client".to_string()) {
        client::run(config)?;
        return Ok(());
    }

    println!("[INFO] DHCP Server starting in mode {:?}...", config.mode);

    // Création du pool SQLite
    let manager = SqliteConnectionManager::file("dhcp.db");
    let pool = Pool::new(manager)?;
    {
        let conn = pool.get()?;
        init_db(&conn)?;
    }

    let dhcp_server = DhcpServer::new(pool);
    let bind_address = format!("{}:{}", config.interface, config.port);
    let socket = UdpSocket::bind(&bind_address).await?;
    socket.set_broadcast(true)?;

    println!("[INFO] Listening on {}...", bind_address);

    // Start API server in a separate task
    let api_handle = task::spawn_blocking(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                if let Err(e) = api::start_api_server().await {
                    eprintln!("[ERROR] API server error: {}", e);
                }
            })
    });

    // Run DHCP server
    match dhcp_server.run(socket).await {
        Ok(_) => println!("[INFO] Server stopped normally."),
        Err(e) => eprintln!("[ERROR] Server error: {}", e),
    }

    // Wait for API server to complete (it won't unless there's an error)
    let _ = api_handle.await;

    Ok(())
}
