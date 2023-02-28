use anyhow::Result;
use saga::client::Client;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::main;
use tracing::{subscriber, Level};

#[main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    subscriber::set_global_default(subscriber)?;
    let client = Client::new("127.0.0.1:8881".parse()?)?;
    let res = client
        .get(
            "somecid".to_string(),
            SocketAddr::from_str("127.0.0.1:4455").unwrap(),
        )
        .await?;
    let mut file = File::create("/Users/acadia/basic-saga.car")?;
    file.write_all(&res)?;
    Ok(())
}
