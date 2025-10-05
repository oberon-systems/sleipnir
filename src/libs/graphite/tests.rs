use super::*;
use serial_test::serial;

#[test]
#[serial]
fn test_graphite_parse_no_tags() {
    let line = "cpu.usage 42.5 1234567890";
    let result = GraphiteMetric::parse(line);

    assert!(result.is_ok());
    let metric = result.unwrap();
    assert_eq!(metric.name, "cpu.usage");
    assert_eq!(metric.value, 42.5);
    assert_eq!(metric.timestamp, 1234567890);
    assert!(metric.tags.is_empty());
}

#[test]
#[serial]
fn test_graphite_parse_with_tags() {
    let line = "cpu.usage;host=server1;app=my_server 42.5 1234567890";
    let result = GraphiteMetric::parse(line);

    assert!(result.is_ok());
    let metric = result.unwrap();
    assert_eq!(metric.name, "cpu.usage");
    assert_eq!(metric.value, 42.5);
    assert_eq!(metric.timestamp, 1234567890);
    assert_eq!(metric.tags.len(), 2);
    assert_eq!(metric.tags.get("host"), Some(&"server1".to_string()));
    assert_eq!(metric.tags.get("app"), Some(&"my_server".to_string()));
}
