use anyhow::Result;
use async_trait::async_trait;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[async_trait]
pub trait Provider: Clone + Sync + Send + 'static {
    async fn get(&self, id: Vec<u8>) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub struct FileProvider(Vec<u8>);

impl FileProvider {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(Self(buf))
    }
}

#[async_trait]
impl Provider for FileProvider {
    async fn get(&self, _: Vec<u8>) -> Option<Vec<u8>> {
        Some(self.0.clone())
    }
}
