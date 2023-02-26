use crate::{protocol, tls};
use anyhow::Result;
use quinn::{Connecting, Endpoint, RecvStream, SendStream, ServerConfig};
use std::{net::SocketAddr, sync::Arc};
use tokio::select;
use tracing::debug;

pub async fn start(bind_addr: SocketAddr) -> Result<()> {
    let config = tls::server_config();
    let endpoint = Endpoint::server(ServerConfig::with_crypto(Arc::new(config)), bind_addr)?;
    loop {
        select! {
            Some(connecting) = endpoint.accept() => {
                tokio::spawn(handle_connection(connecting));
            }
            else => break
        }
    }
    Ok(())
}

async fn handle_connection(connecting: Connecting) {
    if let Ok(connection) = connecting.await {
        while let Ok((tx_stream, rx_stream)) = connection.accept_bi().await {
            tokio::spawn(async move {
                if let Err(err) = handle_stream(tx_stream, rx_stream).await {
                    debug!("error: {err:#?}",);
                }
            });
        }
    } else {
        debug!("Error while connecting");
    }
}

async fn handle_stream(mut tx_stream: SendStream, mut rx_stream: RecvStream) -> Result<()> {
    let mut buf = Vec::new();
    protocol::read(&mut rx_stream, &mut buf).await?;
    debug!("Received {}", String::from_utf8_lossy(buf.as_ref()));
    protocol::write(&mut tx_stream, b"hello client").await?;
    Ok(())
}
