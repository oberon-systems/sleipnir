use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    // create new instance
    pub async fn new(host: &str, port: &str) -> std::io::Result<Self> {
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr).await?;

        log::debug!("listen at {}:{}", host, port);
        Ok(TcpServer { listener })
    }

    // run instance with message handler
    pub async fn run<F>(&self, handler: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let handler = Arc::new(handler);

        loop {
            match self.listener.accept().await {
                Ok((stream, _)) => {
                    let handler_clone = Arc::clone(&handler);

                    // spawn one task per client connection
                    tokio::spawn(async move {
                        Self::handle_client(stream, handler_clone).await;
                    });
                }
                Err(e) => {
                    log::error!("unable to handle a new client: {}", e);
                }
            }
        }
    }

    // a client connection handler - processes all messages from one client
    async fn handle_client<F>(stream: TcpStream, handler: Arc<F>)
    where
        F: Fn(String),
    {
        let peer_addr = stream.peer_addr().unwrap();
        log::debug!("connected: {}", peer_addr);

        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        // process all messages from this client in this one task
        while let Ok(Some(data)) = lines.next_line().await {
            log::debug!("received: {}", data);
            handler(data);
        }

        log::debug!("disconnected: {}", peer_addr);
    }
}

#[cfg(test)]
mod tests;
