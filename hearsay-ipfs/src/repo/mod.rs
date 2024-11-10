//! IPFS repository implementation

use std::sync::{atomic::AtomicUsize, Arc};

use cid::Cid;

use crate::Block;
mod config;
pub use config::Config;
/*
pub trait KvStore<K, V>: Send + Sync {
    async fn contains(&self, key: &K) -> Result<bool, Error>;
    // TODO: async fn list(&self) -> todo!();
    async fn get(&self, key: &K) -> Result<Option<V>, Error>;
    async fn put(&self, key: &K, value: &V) -> Result<(), Error>;
    async fn remove(&self, key: &K) -> Result<(), Error>;
}
*/
#[derive(Clone)]
pub struct Repository {
    pub(crate) inner: Arc<RepoInner>,
}

pub(crate) struct RepoInner {
    pub(crate) capacity: AtomicUsize, // TODO: not pub
    //block_store: Box<dyn KvStore<Cid, Block>>,
}

impl Repository {
    /// Graceful shutdown
    pub fn shutdown(&self) {
        todo!()
    }
}

