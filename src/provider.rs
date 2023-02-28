use anyhow::Result;
use async_trait::async_trait;
use blake3::Hash;
use std::{fs::File, io::Read, path::Path};

#[async_trait]
pub trait Provider: Clone + Sync + Send + 'static {
    async fn get(&self, id: Vec<u8>) -> Option<Data>;
}

#[derive(Clone)]
pub struct FileProvider(Data);

impl FileProvider {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let data = buf.clone();
        let (encoded, hash) = abao::encode::outboard(buf);
        Ok(Self(Data {
            hash,
            outboard: encoded,
            data,
        }))
    }
}

#[async_trait]
impl Provider for FileProvider {
    async fn get(&self, _: Vec<u8>) -> Option<Data> {
        Some(self.0.clone())
    }
}

#[derive(Clone)]
pub struct Data {
    pub hash: Hash,
    pub outboard: Vec<u8>,
    pub data: Vec<u8>,
}
