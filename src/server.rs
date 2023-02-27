use crate::{protocol, provider::Provider, tls};
use anyhow::{anyhow, Result};
use quinn::{Connecting, Endpoint, RecvStream, SendStream, ServerConfig};
use std::{net::SocketAddr, sync::Arc};
use tokio::select;
use tracing::debug;

pub async fn start<P: Provider>(bind_addr: SocketAddr, provider: P) -> Result<()> {
    let config = tls::server_config();
    let endpoint = Endpoint::server(ServerConfig::with_crypto(Arc::new(config)), bind_addr)?;
    loop {
        select! {
            Some(connecting) = endpoint.accept() => {
                tokio::spawn(handle_connection(connecting, provider.clone()));
            }
            else => break
        }
    }
    Ok(())
}

async fn handle_connection<P: Provider>(connecting: Connecting, provider: P) {
    if let Ok(connection) = connecting.await {
        while let Ok((tx_stream, rx_stream)) = connection.accept_bi().await {
            let provider = provider.clone();
            tokio::spawn(async move {
                if let Err(err) = handle_stream(tx_stream, rx_stream, provider).await {
                    debug!("error: {err:#?}",);
                }
            });
        }
    } else {
        debug!("Error while connecting");
    }
}

async fn handle_stream<P: Provider>(
    mut tx_stream: SendStream,
    mut rx_stream: RecvStream,
    provider: P,
) -> Result<()> {
    let mut buf = Vec::new();
    protocol::read(&mut rx_stream, &mut buf).await?;
    debug!("Received {}", String::from_utf8_lossy(buf.as_ref()));
    let content = provider
        .get(buf)
        .await
        .ok_or_else(|| anyhow!("Failed to get data"))?;
    protocol::write(&mut tx_stream, &content).await?;
    Ok(())
}
