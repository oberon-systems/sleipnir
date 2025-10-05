use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GraphiteMetric {
    pub name: String,
    pub tags: HashMap<String, String>,
    pub value: f64,
    pub timestamp: i64,
}

/*
    Parse metric, expected metric format is
    metric_name;tag1=val1;tag2=val2 value timestamp
*/
impl GraphiteMetric {
    pub fn parse(line: &str) -> Result<Self, String> {
        // sanitize and split
        let parts: Vec<&str> = line.split_whitespace().collect();

        // validate metric in format `metric value timestamp`
        if parts.len() != 3 {
            return Err("invalid metric format: expected 3 parts".to_string());
        }

        // collect metrics part
        let metric = parts[0];

        // collect value part
        let value: f64 = parts[1].parse().map_err(|_| "invalid value".to_string())?;

        // collect timestamp
        let timestamp: i64 = parts[2]
            .parse()
            .map_err(|_| "invalid timestamp".to_string())?;

        // process metric
        let mut split = metric.split(';');

        // collect metric name
        let name = split.next().ok_or("missing metric name")?.to_string();

        // map tags
        let mut tags = HashMap::new();
        for tag in split {
            let tag_parts: Vec<&str> = tag.split('=').collect();
            if tag_parts.len() == 2 {
                tags.insert(tag_parts[0].to_string(), tag_parts[1].to_string());
            }
        }

        // return processed data
        Ok(GraphiteMetric {
            name,
            tags,
            value,
            timestamp,
        })
    }
}

#[cfg(test)]
mod tests;
