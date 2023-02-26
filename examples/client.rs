use anyhow::Result;
use saga::client::Client;
use tokio::main;

#[main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = Client::new("127.0.0.1:8881".parse()?)?;
    client.get("somecid".to_string()).await
}
