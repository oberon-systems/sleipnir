use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::libs::graphite::GraphiteMetric;

fn hash_string(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    format!("obf_{:x}", hash)
}

pub fn obfuscate(metric: &GraphiteMetric) -> (String, f64, i64) {
    let name = hash_string(&metric.name);

    let mut result = name;

    for (key, value) in &metric.tags {
        let hashed_value = hash_string(value);
        result.push_str(&format!(";{}={}", key, hashed_value));
    }

    (result, metric.value, metric.timestamp)
}

#[allow(dead_code)]
pub fn obf_to_string(obfuscated: (String, f64, i64)) -> String {
    format!("{} {} {}", obfuscated.0, obfuscated.1, obfuscated.2)
}

#[cfg(test)]
mod tests;
