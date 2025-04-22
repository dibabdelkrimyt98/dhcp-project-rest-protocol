use std::net::UdpSocket;
use std::time::Duration;

pub fn send_discover() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:68")?;
    socket.set_broadcast(true)?;
    socket.set_write_timeout(Some(Duration::from_secs(1)))?;
    socket.send_to(b"DISCOVER", "255.255.255.255:67")?;
    println!("DISCOVER message sent.");
    Ok(())
}
