mod libs;

use libs::ch::{ClickHouseWriter, Metric};
use libs::config;
use libs::graphite;
use libs::obf;
use libs::server;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = config::load();
    log::debug!("configuration: {:?}", config);

    log::info!("starting...");

    // init database writer
    let writer = Arc::new(ClickHouseWriter::new(
        &config.ch_url,
        &config.ch_database,
        &config.ch_username,
        &config.ch_password,
        &config.ch_table,
    ));

    let batch_buffer: Arc<Mutex<Vec<Metric>>> = Arc::new(Mutex::new(Vec::new()));
    let batch_size = config.batch_size;
    let flush_interval = config.flush_interval;

    // required for new flush
    let writer_clone = Arc::clone(&writer);
    let buffer_clone = Arc::clone(&batch_buffer);

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(flush_interval as u64));

        loop {
            interval.tick().await;

            let mut buffer = buffer_clone.lock().await;

            if !buffer.is_empty() {
                log::info!("flushing {} metrics to ClickHouse", buffer.len());

                match writer_clone.batch(buffer.drain(..).collect()).await {
                    Ok(_) => log::debug!("batch flushed successfully"),
                    Err(e) => log::error!("failed to flush batch: {}", e),
                }
            }
        }
    });

    // create server
    let server = server::TcpServer::new(&config.host, &config.port.to_string())
        .await
        .unwrap_or_else(|e| {
            log::error!("unable to create a server: {}", e);
            std::process::exit(1);
        });

    // clone for provide into server async threads
    let writer_for_server = Arc::clone(&writer);
    let buffer_for_server = Arc::clone(&batch_buffer);

    // run server
    server
        .run(move |message| {
            let writer = Arc::clone(&writer_for_server);
            let buffer = Arc::clone(&buffer_for_server);

            let handle = tokio::spawn(async move {
                log::info!("received: {}", message);

                // parse provided metric and obfuscate it and process to CH
                match graphite::GraphiteMetric::parse(&message.to_string()) {
                    Ok(metric) => {
                        log::debug!("parsed metric: {:?}", metric);

                        let obf_metric = obf::obfuscate(&metric);
                        log::debug!("obf metric: {:?}", obf_metric);

                        // convert metrics for ch writer
                        let ch_metric = Metric {
                            path: obf_metric.0,
                            value: obf_metric.1,
                            timestamp: obf_metric.2,
                        };

                        log::debug!("created ch_metric: {:?}", ch_metric);

                        // add into buffer
                        let mut buf = buffer.lock().await;
                        log::debug!("acquired buffer lock, current size: {}", buf.len());
                        buf.push(ch_metric);
                        log::debug!("pushed metric, new size: {}", buf.len());

                        // flush if buffer is full
                        if buf.len() >= batch_size as usize {
                            log::info!("batch size reached, flushing {} metrics", buf.len());

                            match writer.batch(buf.drain(..).collect()).await {
                                Ok(_) => log::debug!("batch written successfully"),
                                Err(e) => log::error!("failed to write batch: {}", e),
                            }
                        } else {
                            log::debug!("buffer not full yet: {}/{}", buf.len(), batch_size);
                        }
                    }
                    Err(e) => {
                        log::error!("failed to parse metric: {}", e);
                    }
                }
            });

            // log if thread has been crashed
            tokio::spawn(async move {
                if let Err(e) = handle.await {
                    log::error!("spawned task panicked: {:?}", e);
                }
            });
        })
        .await;

    // flush before exit for do not loose metrics
    let mut buffer = batch_buffer.lock().await;
    if !buffer.is_empty() {
        log::info!("final flush: {} metrics", buffer.len());
        let _ = writer.batch(buffer.drain(..).collect()).await;
    }

    log::info!("stopped");
}
