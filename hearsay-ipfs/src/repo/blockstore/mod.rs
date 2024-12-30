use async_trait::async_trait;
use cid::Cid;

use crate::Block;

use super::RepoError;

mod mem;

#[async_trait]
pub trait BlockStore: Send + Sync {

    async fn contains(&self, cid: &Cid) -> Result<bool, RepoError>;

    async fn get(&self, cid: &Cid) -> Result<Block, RepoError>;

    async fn get_many(&self, cids: &[&Cid]) -> Result<Vec<Block>, RepoError>;

    async fn put(&self, block: Block) -> Result<(), RepoError>;

    async fn remove(&self, cid: &Cid) -> Result<(), RepoError>;

    async fn remove_many(&self, cids: &[&Cid]) -> Result<(), RepoError>;

    async fn list(&self) -> Result<Vec<Cid>, RepoError>;
}
