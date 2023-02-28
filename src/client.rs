use crate::{protocol, tls};
use abao::decode::AsyncSliceDecoder;
use anyhow::Result;
use blake3::Hash;
use quinn::{ClientConfig, Connection, Endpoint};
use std::{collections::HashMap, net::SocketAddr, str::FromStr, sync::Arc};
use tokio::{io::AsyncReadExt, sync::RwLock};
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

    pub async fn get(&self, id: String) -> Result<Vec<u8>> {
        let client = self.clone();
        let connections = client.pool.0.read().await;
        // TODO: Check if the connection is still valid.
        let connection = match connections.get(&id) {
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
                connections.insert(id.clone(), connection.clone());
                connection
            }
        };
        let (mut tx_stream, mut rx_stream) = connection.open_bi().await.unwrap();
        protocol::write(&mut tx_stream, id.as_bytes()).await?;

        let mut hash = Vec::new();
        protocol::read(&mut rx_stream, &mut hash).await?;

        let hash: [u8; 32] = hash.as_slice().try_into()?;
        let hash = Hash::from(hash);
        debug!("Hash received {hash:?}");

        let mut decoder = AsyncSliceDecoder::new(rx_stream, &hash, 0, u64::MAX);
        let mut decoded = Vec::with_capacity(4096);
        decoder.read_to_end(&mut decoded).await?;
        debug!("File received with size {}", decoded.len());
        Ok(decoded)
    }
}

#[derive(Clone, Default)]
pub struct Pool(Arc<RwLock<HashMap<String, Connection>>>);
