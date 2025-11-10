mod libs;

use libs::ch::{ClickHouseWriter, Metric};
use libs::config;
use libs::graphite;
use libs::obf;
use libs::server;

use std::sync::Arc;
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = config::load();
    log::debug!("configuration: {:?}", config);

    log::info!("starting...");

    let writer = Arc::new(ClickHouseWriter::new(
        &config.ch_url,
        &config.ch_database,
        &config.ch_username,
        &config.ch_password,
        &config.ch_table,
    ));

    let batch_size = config.batch_size;
    let flush_interval = config.flush_interval;

    let (tx, rx) = flume::bounded::<String>(config.channel_buffer.try_into().unwrap());
    let num_workers = config.num_workers;

    for worker_id in 0..num_workers {
        let rx = rx.clone();
        let writer = Arc::clone(&writer);

        tokio::spawn(async move {
            log::info!("worker {} started", worker_id);

            let mut local_buffer = Vec::with_capacity(batch_size as usize);
            let mut last_flush = Instant::now();
            let mut processed = 0;

            loop {
                let elapsed = last_flush.elapsed();
                let timeout_duration = Duration::from_secs(flush_interval as u64);

                let mut got_messages = false;

                for _ in 0..1000 {
                    match rx.try_recv() {
                        Ok(msg) => {
                            got_messages = true;
                            processed += 1;

                            match graphite::GraphiteMetric::parse(&msg) {
                                Ok(metric) => {
                                    let obf_metric = obf::obfuscate(&metric);

                                    let ch_metric = Metric {
                                        path: obf_metric.0,
                                        value: obf_metric.1,
                                        timestamp: obf_metric.2,
                                    };

                                    local_buffer.push(ch_metric);

                                    if local_buffer.len() >= batch_size as usize {
                                        log::info!(
                                            "[{}]: flushing {} metrics (batch full), total processed: {}",
                                            worker_id,
                                            local_buffer.len(),
                                            processed
                                        );

                                        match writer.batch(std::mem::take(&mut local_buffer)).await
                                        {
                                            Ok(_) => {
                                                log::debug!(
                                                    "[{}]: batch written successfully",
                                                    worker_id
                                                );
                                                last_flush = Instant::now();
                                            }
                                            Err(e) => log::error!(
                                                "[{}]: failed to write batch: {}",
                                                worker_id,
                                                e
                                            ),
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("[{}]: failed to parse metric: {}", worker_id, e);
                                }
                            }
                        }
                        Err(flume::TryRecvError::Empty) => break,
                        Err(flume::TryRecvError::Disconnected) => {
                            if !local_buffer.is_empty() {
                                log::info!(
                                    "[{}]: final flush {} metrics",
                                    worker_id,
                                    local_buffer.len()
                                );
                                let _ = writer.batch(std::mem::take(&mut local_buffer)).await;
                            }
                            log::info!(
                                "worker {} stopped, processed {} total",
                                worker_id,
                                processed
                            );
                            return;
                        }
                    }
                }

                if !got_messages {
                    if elapsed >= timeout_duration && !local_buffer.is_empty() {
                        log::info!(
                            "[{}]: flushing {} metrics (timeout)",
                            worker_id,
                            local_buffer.len()
                        );

                        match writer.batch(std::mem::take(&mut local_buffer)).await {
                            Ok(_) => {
                                log::debug!("[{}]: batch written successfully", worker_id);
                                last_flush = Instant::now();
                            }
                            Err(e) => log::error!("[{}]: failed to write batch: {}", worker_id, e),
                        }
                    } else {
                        tokio::task::yield_now().await;
                    }
                }
            }
        });
    }

    let server = server::TcpServer::new(&config.host, &config.port.to_string())
        .await
        .unwrap_or_else(|e| {
            log::error!("unable to create a server: {}", e);
            std::process::exit(1);
        });

    server
        .run(move |message| {
            let tx = tx.clone();

            if let Err(e) = tx.try_send(message) {
                log::error!("channel full, dropping message: {}", e);
            }
        })
        .await;

    log::info!("stopped");
}
