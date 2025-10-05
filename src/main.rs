mod libs;

use libs::config;
use libs::graphite;
use libs::server;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = config::load();
    log::debug!("configuration: {:?}", config);

    let server =
        server::TcpServer::new(&config.host, &config.port.to_string()).unwrap_or_else(|e| {
            log::error!("unable to create a server: {}", e);
            std::process::exit(1);
        });

    server.run(|message| {
        log::info!("Received: {}", message);

        match graphite::GraphiteMetric::parse(&message.to_string()) {
            Ok(metric) => {
                log::debug!("Parsed metric: {:?}", metric);
            }
            Err(e) => {
                log::error!("failed to parse metric: {}", e);
            }
        }
    });
}
