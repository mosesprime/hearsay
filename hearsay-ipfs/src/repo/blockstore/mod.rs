use async_trait::async_trait;
use cid::Cid;
use libp2p::futures::stream::BoxStream;

use crate::Block;

mod mem;

#[async_trait]
pub trait BlockStore: Send + Sync {
    async fn contains(&self, cid: &Cid) -> Result<bool, Error>;
    async fn list(&self) -> BoxStream<Cid>;
    async fn get(&self, cid: &Cid) -> Result<Option<Block>, Error>;
    async fn put(&self, block: &Block) -> Result<(), Error>;
    async fn remove(&self, cid: &Cid) -> Result<(), Error>;
}
