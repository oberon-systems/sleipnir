use super::*;
use serial_test::serial;
use tools::EnvSetter;

#[test]
#[serial]
fn test_config_load_all_values() {
    let mut env = EnvSetter::new();
    env.set("APP_HOST", "example.com");
    env.set("APP_PORT", "3000");
    env.set("APP_CH_URL", "ch.example.com");
    env.set("APP_CH_PASSWORD", "password");

    env.set("APP_LABELS_CIRCUIT", "test-circuit");
    env.set("APP_LABELS_ENV", "test");
    env.set("APP_LABELS_PROJECT", "test-project");

    let config = load();

    assert_eq!(config.host, "example.com");
    assert_eq!(config.ch_url, "ch.example.com");
    assert_eq!(config.ch_password, "password");
    assert_eq!(config.port, 3000);
}

#[test]
#[serial]
fn test_config_load_defaults() {
    let mut env = EnvSetter::new();
    env.del("APP_HOST");
    env.del("APP_PORT");

    env.set("APP_CH_URL", "ch.example.com");
    env.set("APP_CH_PASSWORD", "password");

    env.set("APP_LABELS_CIRCUIT", "test-circuit");
    env.set("APP_LABELS_ENV", "test");
    env.set("APP_LABELS_PROJECT", "test-project");

    let config = load();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
}
