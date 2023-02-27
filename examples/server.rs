use anyhow::Result;
use tokio::main;
use tracing::{subscriber, Level};

#[main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    subscriber::set_global_default(subscriber)?;
    saga::server::start("0.0.0.0:4455".parse()?).await?;
    Ok(())
}
