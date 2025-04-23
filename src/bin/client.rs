use tokio::net::UdpSocket;
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("DHCP Client starting...");
    
    // Create UDP socket for DHCP client (port 68)
    let socket = UdpSocket::bind("0.0.0.0:68").await?;
    socket.set_broadcast(true)?;
    
    // Send DISCOVER message
    println!("Sending DISCOVER message to 255.255.255.255:6767...");
    let discover = json!({
        "msg_type": "DISCOVER"
    }).to_string();
    
    socket.send_to(discover.as_bytes(), "255.255.255.255:6767").await?;
    
    // Wait for OFFER response
    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf).await?;
    let response = std::str::from_utf8(&buf[..amt]).unwrap_or("");
    println!("Received response from {}: {}", src, response);
    
    Ok(())
}