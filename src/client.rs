use crate::{protocol, tls};
use anyhow::Result;
use quinn::{ClientConfig, Connection, Endpoint};
use std::{collections::HashMap, net::SocketAddr, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tracing::debug;

#[derive(Clone)]
pub struct Client {
    endpoint: Endpoint,
    pool: Pool,
}

impl Client {
    pub fn new(bind_addr: SocketAddr) -> Result<Self> {
        let config = tls::client_config();
        let mut endpoint = Endpoint::client(bind_addr)?;
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(config)));
        Ok(Self {
            endpoint,
            pool: Pool::default(),
        })
    }

    pub async fn get(&self, cid: String) -> Result<()> {
        let client = self.clone();
        let connections = client.pool.0.read().await;
        let connection = match connections.get(&cid) {
            Some(connection) => connection.clone(),
            None => {
                drop(connections);
                let connection = client
                    .endpoint
                    .connect(
                        SocketAddr::from_str("127.0.0.1:4455").unwrap(),
                        "servername",
                    )?
                    .await?;
                let mut connections = client.pool.0.write().await;
                connections.insert(cid.clone(), connection.clone());
                connection
            }
        };
        let (mut tx_stream, mut rx_stream) = connection.open_bi().await.unwrap();
        protocol::write(&mut tx_stream, b"hello").await?;

        let mut buf = Vec::new();
        protocol::read(&mut rx_stream, &mut buf).await?;

        debug!("{}", String::from_utf8_lossy(buf.as_ref()));
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct Pool(Arc<RwLock<HashMap<String, Connection>>>);
