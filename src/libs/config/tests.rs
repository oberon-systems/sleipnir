use super::*;
use serial_test::serial;
use tools::EnvSetter;

#[test]
#[serial]
fn test_config_load_all_values() {
    let mut env = EnvSetter::new();
    env.set("APP_HOST", "example.com");
    env.set("APP_PORT", "3000");

    let config = load();

    assert_eq!(config.host, "example.com");
    assert_eq!(config.port, 3000);
}

#[test]
#[serial]
fn test_config_load_defaults() {
    let mut env = EnvSetter::new();
    env.del("APP_HOST");
    env.del("APP_PORT");

    let config = load();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
}
