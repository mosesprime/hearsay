//! IPFS repository implementation

use std::sync::{atomic::AtomicUsize, Arc};

use async_trait::async_trait;
use blockstore::BlockStore;
use cid::Cid;
use libp2p::futures::stream::BoxStream;

mod blockstore;
mod config;
pub use config::Config;

#[async_trait]
pub trait DataStore: PinStore + Send + Sync {
    async fn contains(&self, key: &[u8]) -> Result<bool, Error>;
    // TODO: async fn list(&self) -> todo!();
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error>;
    async fn remove(&self, key: &[u8]) -> Result<(), Error>;
}

#[async_trait]
pub trait PinStore: Send + Sync {
    async fn is_pinned(&self, cid: &Cid) -> Result<bool, Error>;
    async fn insert_pin(&self, cid: &Cid) -> Result<(), Error>;
    async fn remove_pin(&self, cid: &Cid) -> Result<(), Error>;
    async fn list_pins(&self) -> BoxStream<Result<Cid, Error>>;
}

#[derive(Clone)]
pub struct Repository {
    pub(crate) inner: Arc<RepoInner>,
}

pub(crate) struct RepoInner {
    capacity: AtomicUsize,
    data_store: Box<dyn DataStore>,
    block_store: Box<dyn BlockStore>,

}

impl Repository {
    /// Graceful shutdown
    pub fn shutdown(&self) {
        todo!()
    }
}

