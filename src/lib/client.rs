use clickhouse::{error::Result, Client, Row};
use crate::batch::Batch;

pub struct BufferClient<'a> {
    ch_client: &'a Client,
    table: &'a str,
}

impl<'a> BufferClient<'a> {
    pub fn new(client: &'a Client, table: &'a str) -> Self {
        BufferClient{ ch_client: client, table }
    }

    pub async fn write<T>(&self, rows: Vec<T>) -> Result<()>
        where T: Row + serde::ser::Serialize {
        Clickhouse::insert(self.ch_client, self.table, Batch::new(rows)).await
    }
}

struct Clickhouse;

impl Clickhouse {
    async fn insert<T>(client: &Client, table: &str, batch: Batch<T>) -> Result<()>
        where T: Row + serde::ser::Serialize {
        let mut insert = client.insert(table)?;
        for i in batch.rows {
            insert.write(&i).await?;
        }

        insert.end().await
    }
}