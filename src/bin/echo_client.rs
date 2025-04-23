use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("UDP Echo Client starting...");
    
    // Create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    println!("Client bound to {}", socket.local_addr()?);
    
    // Message to send
    let message = "Hello, UDP Echo Server!";
    println!("Sending: {}", message);
    
    // Send message to server
    socket.send_to(message.as_bytes(), "127.0.0.1:6767").await?;
    
    // Wait for response
    let mut buf = [0; 1024];
    let (size, src) = socket.recv_from(&mut buf).await?;
    let response = std::str::from_utf8(&buf[..size]).unwrap_or("");
    println!("Received from {}: {}", src, response);
    
    Ok(())
} 