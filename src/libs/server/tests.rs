use super::*;
use serial_test::serial;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::test]
#[serial]
async fn test_server_bind() {
    let server = TcpServer::new("127.0.0.1", "0").await;
    assert!(server.is_ok(), "server should start");
}

#[tokio::test]
#[serial]
async fn test_server_client_connection() {
    let server = TcpServer::new("127.0.0.1", "0").await.unwrap();
    let addr = server.listener.local_addr().unwrap();

    // spawn server in a new task
    tokio::spawn(async move {
        server.run(|_| {}).await;
    });

    // sleep for server could start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // connect to server
    let client = TcpStream::connect(addr).await;
    assert!(client.is_ok(), "client should connect");
}

#[tokio::test]
#[serial]
async fn test_server_message_handling() {
    let server = TcpServer::new("127.0.0.1", "0").await.unwrap();
    let addr = server.listener.local_addr().unwrap();

    let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_clone = received.clone();

    // spawn server
    tokio::spawn(async move {
        server
            .run(move |msg| {
                received_clone.lock().unwrap().push(msg);
            })
            .await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // connect and send message
    let mut client = TcpStream::connect(addr).await.unwrap();
    client.write_all(b"test message\n").await.unwrap();

    tokio::time::sleep(Duration::from_millis(100)).await;

    let messages = received.lock().unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0], "test message");
}
