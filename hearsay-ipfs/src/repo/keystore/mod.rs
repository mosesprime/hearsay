use async_trait::async_trait;
use libp2p::futures::stream::BoxStream;

mod mem;

#[async_trait]
pub trait KeyStore: Send + Sync {
    async fn contains(&self, domain: &str) -> Result<bool, Error>;
    async fn list(&self) -> Result<BoxStream<&str>, Error>;
    async fn get(&self, domain: &str) -> Result<&[u8], Error>;
    async fn put(&self, domain: &str, key: &[u8]) -> Result<(), Error>;
    async fn remove(&self, domain: &str) -> Result<(), Error>;
}
