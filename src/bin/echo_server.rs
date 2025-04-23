use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("UDP Echo Server starting on port 6767...");
    
    // Create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:6767").await?;
    println!("Server listening on 0.0.0.0:6767");
    
    let mut buf = [0; 1024];
    
    loop {
        // Receive data
        let (size, src) = socket.recv_from(&mut buf).await?;
        let received = std::str::from_utf8(&buf[..size]).unwrap_or("");
        println!("Received from {}: {}", src, received);
        
        // Echo back the data
        socket.send_to(&buf[..size], src).await?;
        println!("Echoed back to {}", src);
    }
} 