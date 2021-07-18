fn main() {
    println!("{:?}", "Good");
}

#[cfg(test)]
mod tests {
    use std::env;
    use serde::{Deserialize, Serialize};
    use clickhouse::{error::Result, Client, Row};
    use clickhouse_buffer::client::{BufferClient};

    #[derive(Debug, Row, Serialize, Deserialize)]
    struct MyRow<'a> {
        no: u32,
        name: &'a str,
    }

    #[actix_rt::test]
    async fn it_works() {
        let host = env::var("CLICKHOUSE_HOST_20").unwrap();
        let client = Client::default()
            .with_url(format!("{}:8123", host))
            .with_user("default")
            .with_database("default");

        before_check_table(&client).await.unwrap();

        let buffer_client = BufferClient::new(&client, "some");
        let sample = sample_dataset();

        buffer_client.write::<MyRow>(sample).await.unwrap();
    }

    fn sample_dataset<'a>() -> Vec<MyRow<'a>> {
        let mut rows: Vec<MyRow> = vec![];
        for i in 0..5 {
            rows.push(MyRow { no: i, name: "foo" });
        }
        rows
    }

    async fn before_check_table(client: &Client) -> Result<()> {
        drop_table(client).await.unwrap();
        create_table(client).await
    }

    async fn drop_table(client: &Client) -> Result<()> {
        client.query("DROP TABLE IF EXISTS some").execute().await
    }

    async fn create_table(client: &Client) -> Result<()> {
        let ddl = "
        CREATE TABLE some(no UInt32, name LowCardinality(String)) ENGINE = MergeTree ORDER BY no
        ";

        client.query(ddl).execute().await
    }
}