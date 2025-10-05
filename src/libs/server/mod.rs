use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    // create new instance
    pub fn new(host: &str, port: &str) -> std::io::Result<Self> {
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr)?;

        log::debug!("listen at {}:{}", host, port);
        Ok(TcpServer { listener })
    }

    // run instance
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    Self::handle_client(stream);
                }
                Err(e) => {
                    log::error!("unable to handle a new client: {}", e);
                }
            }
        }
    }

    // a client connection handler
    fn handle_client(stream: TcpStream) {
        let peer_addr = stream.peer_addr().unwrap();
        log::debug!("connected: {}", peer_addr);

        let reader = BufReader::new(stream);

        for line in reader.lines() {
            match line {
                Ok(data) => {
                    log::debug!("received: {}", data);
                }
                Err(e) => {
                    log::error!("data read error: {}", e);
                    break;
                }
            }
        }

        log::debug!("disconnected: {}", peer_addr);
    }
}

#[cfg(test)]
mod tests;
