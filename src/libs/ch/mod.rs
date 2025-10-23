use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

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

    pub async fn batch(&self, metrics: Vec<Metric>) -> Result<(), clickhouse::error::Error> {
        let mut insert = self.client.insert::<Metric>(&self.table_name).await?;

        for metric in metrics {
            insert.write(&metric).await?;
        }

        insert.end().await?;
        Ok(())
    }
}
