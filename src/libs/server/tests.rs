use super::*;
use serial_test::serial;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[test]
#[serial]
fn test_server_bind() {
    let server = TcpServer::new("127.0.0.1", "0");
    assert!(server.is_ok(), "server should start");
}

#[test]
#[serial]
fn test_server_client_connection() {
    let server = TcpServer::new("127.0.0.1", "0").unwrap();
    let addr = server.listener.local_addr().unwrap();

    // spawn in a new thread
    thread::spawn(move || {
        if let Ok(stream) = server.listener.accept() {
            TcpServer::handle_client(stream.0, |_| {});
        }
    });

    // sleep for server could start
    thread::sleep(Duration::from_millis(100));

    // connect to server
    let client = TcpStream::connect(addr);
    assert!(client.is_ok(), "client should connect");
}
