use anyhow::Result;
use tokio::main;

#[main]
async fn main() -> Result<()> {
    env_logger::init();
    saga::server::start("0.0.0.0:4455".parse()?).await?;
    Ok(())
}
