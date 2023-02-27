use anyhow::Result;
use saga::client::Client;
use tokio::main;
use tracing::{subscriber, Level};

#[main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    subscriber::set_global_default(subscriber)?;
    let client = Client::new("127.0.0.1:8881".parse()?)?;
    client.get("somecid".to_string()).await
}
