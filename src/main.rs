mod libs;

use libs::ch::{ClickHouseWriter, Metric};
use libs::config;
use libs::graphite;
use libs::obf;
use libs::server;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = config::load();
    log::debug!("configuration: {:?}", config);

    log::info!("starting...");

    let (tx, rx) = flume::bounded::<String>(config.channel_buffer.try_into().unwrap());
    let num_workers = config.num_workers;

    for worker_id in 0..num_workers {
        let rx = rx.clone();
        let batch_size = config.batch_size;
        let flush_interval = config.flush_interval;

        let ch_url = config.ch_url.clone();
        let ch_database = config.ch_database.clone();
        let ch_username = config.ch_username.clone();
        let ch_password = config.ch_password.clone();
        let ch_table = config.ch_table.clone();

        tokio::spawn(async move {
            log::info!("worker {} started", worker_id);

            let writer =
                ClickHouseWriter::new(&ch_url, &ch_database, &ch_username, &ch_password, &ch_table);
            log::info!("[{}]: created writer", worker_id);

            let mut inserter = writer.create_inserter(batch_size.into(), flush_interval.into());
            log::info!("[{}]: created inserter", worker_id);

            let mut processed: u64 = 0;

            loop {
                match rx.recv_async().await {
                    Ok(msg) => {
                        processed = processed.checked_add(1).unwrap_or_else(|| {
                            log::error!("[{}]: counter overflow: resetting to 0", worker_id);
                            1 // return (set) 1 and start again
                        });

                        if processed % 10000_u64 == 0 {
                            log::info!("[{}]: processed {} metrics", worker_id, processed);
                        }

                        if processed % batch_size as u64 == 0 {
                            match inserter.commit().await {
                                Ok(_) => {
                                    log::info!(
                                        "[{}]: inserter: written {} strings",
                                        worker_id,
                                        batch_size
                                    )
                                }
                                Err(e) => {
                                    log::error!(
                                        "[{}]: inserter: unable to commit: {}",
                                        worker_id,
                                        e
                                    )
                                }
                            }
                        }

                        match graphite::GraphiteMetric::parse(&msg) {
                            Ok(metric) => {
                                let mut buf = [0u8; obf::MAX_METRIC_LEN];
                                let obf_path = obf::obfuscate(&metric, &mut buf);
                                let obf_metric = Metric {
                                    path: obf_path.to_string(),
                                    value: metric.value,
                                    timestamp: metric.timestamp,
                                };

                                log::debug!(
                                    "[{}]: obf metric: {} {} {}",
                                    worker_id,
                                    obf_metric.path,
                                    obf_metric.value,
                                    obf_metric.timestamp
                                );

                                if let Err(e) = inserter.write(&obf_metric).await {
                                    log::error!("[{}]: failed to write metric: {}", worker_id, e);
                                }
                            }
                            Err(e) => {
                                log::error!("[{}]: failed to parse metric: {}", worker_id, e);
                            }
                        }
                    }
                    Err(_) => {
                        log::info!("[{}]: total processed: {}", worker_id, processed);
                        return;
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
