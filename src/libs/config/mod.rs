mod tools;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

/// Defaults
fn default_host() -> String {
    "localhost".to_string()
}
fn default_port() -> u16 {
    8080
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
