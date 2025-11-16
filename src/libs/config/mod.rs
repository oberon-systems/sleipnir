mod tools;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PrometheusLabels {
    pub application: String,
    pub circuit: String,
    pub env: String,
    pub project: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    // required params
    pub ch_url: String,
    pub ch_password: String,

    // optional params
    #[serde(default = "default_num_workers")]
    pub num_workers: u8,
    #[serde(default = "default_channel_buffer")]
    pub channel_buffer: u32,
    #[serde(default = "default_flush_interval")]
    pub flush_interval: u8,
    #[serde(default = "default_batch_size")]
    pub batch_size: u16,
    #[serde(default = "default_username")]
    pub ch_username: String,
    #[serde(default = "default_table")]
    pub ch_table: String,
    #[serde(default = "default_database")]
    pub ch_database: String,
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u8,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,

    // prometheus client
    #[serde(default = "default_prometheus_labels")]
    pub labels: PrometheusLabels,
    #[serde(default = "default_prometheus_host")]
    pub prometheus_host: String,
    #[serde(default = "default_prometheus_port")]
    pub prometheus_port: u16,
}

/// Defaults
fn default_username() -> String {
    "default".to_string()
}
fn default_table() -> String {
    "metrics".to_string()
}
fn default_database() -> String {
    "sleipnir".to_string()
}
fn default_host() -> String {
    "localhost".to_string()
}
fn default_flush_interval() -> u8 {
    5
}
fn default_batch_size() -> u16 {
    1000
}
fn default_port() -> u16 {
    8080
}
fn default_connection_timeout() -> u8 {
    30
}
fn default_num_workers() -> u8 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u8)
        .unwrap_or(4)
}
fn default_channel_buffer() -> u32 {
    10000
}

// Prometheus Client Defaults
fn default_prometheus_labels() -> PrometheusLabels {
    PrometheusLabels {
        application: "sleipnir".to_string(),
        circuit: "unknown".to_string(),
        env: "unknown".to_string(),
        project: "unknown".to_string(),
    }
}
fn default_prometheus_host() -> String {
    "0.0.0.0".to_string()
}
fn default_prometheus_port() -> u16 {
    9090
}

/*
Load configuration both from .env file
and from environment, configuration params
should be prefixed with `APP_` string.
*/
pub fn load() -> Config {
    dotenvy::dotenv().ok();

    envy::prefixed("APP_").from_env().unwrap()
}

#[cfg(test)]
mod tests;
