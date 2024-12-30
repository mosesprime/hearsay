use async_trait::async_trait;

use super::RepoError;

mod mem;

#[async_trait]
pub trait KeyStore: Send + Sync {

    async fn contains(&self, domain: &str) -> Result<bool, RepoError>;

    async fn list(&self) -> Result<Vec<String>, RepoError>;

    async fn get(&self, domain: &str) -> Result<Vec<u8>, RepoError>;

    async fn put(&self, domain: &str, key: &[u8]) -> Result<(), RepoError>;

    async fn remove(&self, domain: &str) -> Result<(), RepoError>;
}
