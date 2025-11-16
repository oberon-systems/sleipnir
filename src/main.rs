mod libs;

use libs::ch::{ClickHouseWriter, Metric};
use libs::config::{self, PrometheusLabels};
use libs::graphite;
use libs::obf;
use libs::prometheus::Prometheus;
use libs::server;

use axum::{Router, routing::get};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = config::load();
    log::debug!("configuration: {:?}", config);

    log::info!("starting...");

    let (tx, rx) = flume::bounded::<String>(config.channel_buffer.try_into().unwrap());
    let num_workers = config.num_workers;

    // init prometheus client
    let PrometheusLabels {
        application,
        circuit,
        env,
        project,
    } = &config.labels;
    let promc = Arc::new(Prometheus::new(
        application.clone(),
        circuit.clone(),
        env.clone(),
        project.clone(),
    ));

    let promc_main = promc.clone();

    // init exporter (web)
    let promc_web = promc_main.clone();
    let prometheus_host = config.prometheus_host.clone();
    let prometheus_port = config.prometheus_port;

    tokio::spawn(async move {
        let app = Router::new().route("/metrics", get(|| async move { promc_web.export() }));

        let listener =
            tokio::net::TcpListener::bind(format!("{}:{}", prometheus_host, prometheus_port))
                .await
                .unwrap();
        log::info!(
            "Metrics server listening on http://{}:{}/metrics",
            prometheus_host,
            prometheus_port
        );
        axum::serve(listener, app).await.unwrap();
    });

    for worker_id in 0..num_workers {
        let rx = rx.clone();
        let batch_size = config.batch_size;
        let flush_interval = config.flush_interval;

        let ch_url = config.ch_url.clone();
        let ch_database = config.ch_database.clone();
        let ch_username = config.ch_username.clone();
        let ch_password = config.ch_password.clone();
        let ch_table = config.ch_table.clone();

        let promc = promc.clone();
        let labels = promc.worker_id(worker_id.into());

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
                        promc.received.get_or_create(&labels).inc();

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

                                promc.processed.get_or_create(&labels).inc();

                                if let Err(e) = inserter.write(&obf_metric).await {
                                    log::error!("[{}]: failed to write metric: {}", worker_id, e);
                                    promc.errors.get_or_create(&labels).inc();
                                }
                            }
                            Err(e) => {
                                log::error!("[{}]: failed to parse metric: {}", worker_id, e);
                                promc.errors.get_or_create(&labels).inc();
                            }
                        }
                    }
                    Err(_) => {
                        log::info!("[{}]: total processed: {}", worker_id, processed);
                        promc.errors.get_or_create(&promc.labels).inc();
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
            promc_main.errors.get_or_create(&promc_main.labels).inc();
            std::process::exit(1);
        });

    server
        .run(move |message| {
            let tx = tx.clone();

            if let Err(e) = tx.try_send(message) {
                log::error!("channel full, dropping message: {}", e);
                promc_main.dropped.get_or_create(&promc_main.labels).inc();
            }
        })
        .await;

    log::info!("stopped");
}
