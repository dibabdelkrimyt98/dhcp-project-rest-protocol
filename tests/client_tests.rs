use dhcp_project::client;

#[tokio::test]
async fn test_send_discover() {
    use tokio::net::UdpSocket;
    use std::str;

    let listener = UdpSocket::bind("0.0.0.0:67").await.unwrap();

    // Run client in separate task
    tokio::spawn(async {
        client::discovery::send_discover().unwrap();
    });

    let mut buf = [0; 1024];
    let (len, _) = listener.recv_from(&mut buf).await.unwrap();
    let received = str::from_utf8(&buf[..len]).unwrap();
    assert_eq!(received, "DISCOVER");
}
