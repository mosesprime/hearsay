use std::collections::{hash_map::Entry, HashMap};

use async_trait::async_trait;
use bytes::Bytes;
use cid::Cid;
use tokio::sync::RwLock;
use tracing::trace;
use crate::{repo::RepoError, Block};

use super::BlockStore;

pub struct MemBlockStore {
    inner: RwLock<HashMap<Cid, Bytes>>,
}

impl MemBlockStore {
    pub fn new() -> Self {
        Self { 
            inner: RwLock::new(HashMap::default()),
        }
    }
}

#[async_trait]
impl BlockStore for MemBlockStore {
    async fn contains(&self, cid: &Cid) -> Result<bool, RepoError> {
        Ok(self.inner.read().await.contains_key(cid))
    }

    async fn get(&self, cid: &Cid) -> Result<Block, RepoError> {
        let inner = &*self.inner.read().await;
        if let Some(data) = inner.get(cid) {
            let block = Block::new(*cid, data.clone()).map_err(|_| RepoError::IncorrectCid)?;
            return Ok(block);
        }
        Err(RepoError::NotFound)
    }

    async fn get_many(&self, cids: &[&Cid]) -> Result<Vec<Block>, RepoError> {
        let inner = &*self.inner.read().await;
        let mut blocks = vec![];
        for cid in cids {
            if let Some(data) = inner.get(*cid) {
                let block = Block::new(**cid, data.clone()).map_err(|_| RepoError::IncorrectCid)?;
                blocks.push(block);
            } else {
                return Err(RepoError::NotFound);
            }
        }
        Ok(blocks)
    }

    async fn put(&self, block: Block) -> Result<(), RepoError> {
        let inner = &mut *self.inner.write().await;
        match inner.entry(*block.cid()) {
            Entry::Vacant(e) => {
                e.insert(block.inner().clone());
            },
            Entry::Occupied(_) => {
                trace!("block {:?} already exists", block.cid());
            },
        }
        Ok(())
    }

    async fn remove(&self, cid: &Cid) -> Result<(), RepoError> {
        let inner = &mut *self.inner.write().await;
        match inner.remove(cid) {
            Some(_) => Ok(()),
            None => Err(RepoError::NotFound),
        }
    }

    async fn remove_many(&self, cids: &[&Cid]) -> Result<(), RepoError> {
        let inner = &mut *self.inner.write().await;
        for cid in cids {
            if inner.remove(*cid).is_none() {
                return Err(RepoError::NotFound);
            }
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Cid>, RepoError> {
        let inner = &*self.inner.read().await;
        Ok(inner.keys().copied().collect::<Vec<_>>())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mem_block_store() {
        let store = MemBlockStore::new();
        let value = b"banana";
        let block = Block::new(cid, data)

        assert!(!store.contains(key).await.unwrap());
        assert!(store.get(key).await.unwrap() == None);
        store.remove(key).await.unwrap();
        store.put(key, value).await.unwrap();
        assert!(store.contains(key).await.unwrap());
        assert!(store.get(key).await.unwrap() == Some(Bytes::from_static(value)));
        store.remove(key).await.unwrap();
        assert!(!store.contains(key).await.unwrap());
    }
}*/ 
// TODO: add blockstore tests
