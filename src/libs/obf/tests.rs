use super::*;
use serial_test::serial;

use crate::libs::graphite::GraphiteMetric;

#[test]
#[serial]
fn test_obf_name_changed() {
    let metric = GraphiteMetric::parse("cpu.usage 42.5 1234567890").unwrap();

    let (name, _, _) = obfuscate(&metric);

    assert!(name.starts_with("obf_"));
    assert!(!name.contains("cpu.usage"));
}

#[test]
#[serial]
fn test_obf_is_continues() {
    let metric = GraphiteMetric::parse("memory.used 1024.0 1234567890").unwrap();

    let obfuscated1 = obf_to_string(obfuscate(&metric));
    let obfuscated2 = obf_to_string(obfuscate(&metric));

    assert_eq!(obfuscated1, obfuscated2);
}

#[test]
fn test_obf_with_tags() {
    let metric =
        GraphiteMetric::parse("app.requests;host=server01;region=eu-west 150.0 1234567890")
            .unwrap();

    let (name, _, _) = obfuscate(&metric);

    assert!(name.contains(";host=obf_"));
    assert!(name.contains(";region=obf_"));
    assert!(!name.contains("server01"));
    assert!(!name.contains("eu-west"));
}
