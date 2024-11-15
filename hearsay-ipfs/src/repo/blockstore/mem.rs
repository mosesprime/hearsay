use std::{collections::{hash_map::Entry, HashMap}, sync::Arc};

use async_trait::async_trait;
use bytes::Bytes;
use cid::Cid;
use libp2p::futures::{stream::{self, BoxStream}, StreamExt};
use tokio::sync::RwLock;
use tracing::trace; // TODO: better RwLock w/o tokio?

use crate::Block;

use super::BlockStore;

/// In memory [Block] store.
pub struct MemBlockStore {
    inner: Arc<RwLock<HashMap<Cid, Bytes>>>
}

impl MemBlockStore {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }
}

#[async_trait]
impl BlockStore for MemBlockStore {
    async fn contains(&self, cid: &Cid) -> Result<bool, Error> {
        Ok(self.inner.read().await.contains_key(cid))
    }

    async fn list(&self) -> BoxStream<Cid> {
        let inner = &*self.inner.read().await;
        stream::iter(inner.keys().copied().collect::<Vec<_>>()).boxed()
    }

    async fn get(&self, cid: &Cid) -> Result<Option<Block>, Error> {
        let inner = &*self.inner.read().await;
        if let Some(bytes) = inner.get(cid) {
            return Some(Block::new(*cid, bytes.clone())?);
        }
        Ok(None)
    }

    async fn put(&self, block: &Block) -> Result<(), Error> {
        let inner = &mut *self.inner.write().await;
        match inner.entry(*block.cid()) {
            Entry::Occupied(_) => {
                trace!("block {} already exists", block.cid());
                Ok(())
            },
            Entry::Vacant(e) => {
                e.insert(block.inner().clone());
                Ok(())
            },
        }
    }

    async fn remove(&self, cid: &Cid) -> Result<(), Error> {
        let inner = &mut *self.inner.write().await;
        match inner.remove(cid) {
            Some(_) => Ok(()),
            None => Err(todo!()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cid::Cid;
    use multihash_codetable::Code;

    
    #[tokio::test]
    async fn test_mem_blockstore() {
        let store = MemBlockStore::new();
        let data = b"banana";
        let cid = Cid::new_v1(0, Code::Sha2_256.digest(data)); // TODO: use propper codec instead
        let block = Block::new(cid, data.to_vec().into()).unwrap();

        assert!(!store.contains(&cid).await.unwrap());
        assert!(store.get(&cid).await.unwrap().is_none());
        assert!(store.remove(&cid).await.is_err());

        assert!(store.put(&block).await.unwrap().is_ok());
        assert!(store.contains(&cid).await.unwrap());
        assert!(store.get(&cid).await.unwrap() == Some(block.clone()));
        assert!(store.remove(&cid).await.unwrap().is_ok());
        assert!(!store.contains(&cid).await.unwrap());
    }

    // TODO: add test for listing blocks
}
