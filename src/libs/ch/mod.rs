use clickhouse::{Client, Row, inserter::Inserter};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct Metric {
    pub path: String,
    pub value: f64,
    pub timestamp: i64,
}

pub struct ClickHouseWriter {
    client: Client,
    table_name: String,
}

impl ClickHouseWriter {
    pub fn new(
        url: &str,
        database: &str,
        username: &str,
        password: &str,
        table_name: &str,
    ) -> Self {
        let client = Client::default()
            .with_url(url)
            .with_database(database)
            .with_user(username)
            .with_password(password);

        Self {
            client,
            table_name: table_name.to_string(),
        }
    }

    #[allow(dead_code)]
    pub async fn batch(&self, metrics: Vec<Metric>) -> Result<(), clickhouse::error::Error> {
        let mut insert = self.client.insert::<Metric>(&self.table_name).await?;

        for metric in metrics {
            insert.write(&metric).await?;
        }

        insert.end().await?;
        Ok(())
    }

    pub fn create_inserter(&self, max_rows: u64, period_secs: u64) -> Inserter<Metric> {
        self.client
            .inserter::<Metric>(&self.table_name)
            .with_max_rows(max_rows)
            .with_period(Some(Duration::from_secs(period_secs)))
    }
}
