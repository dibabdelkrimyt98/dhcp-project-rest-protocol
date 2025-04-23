use std::net::UdpSocket;
use std::time::Duration;
use crate::config::Config;

pub fn run(config: Config) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.set_write_timeout(Some(Duration::from_secs(1)))?;

    let destination = format!("{}:{}", config.interface, config.port);
    socket.send_to(b"DISCOVER", &destination)?;
    println!("[CLIENT] DISCOVER sent to {}", destination);
    Ok(())
}
