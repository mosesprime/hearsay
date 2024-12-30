//! IPFS repository implementation

use std::sync::Arc;

use blockstore::BlockStore;
use cid::Cid;

pub mod blockstore;
pub mod keystore;
pub mod pinstore;
use keystore::KeyStore;
use pinstore::{PinMode, PinStore};
use thiserror::Error;

use crate::Block;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("CID does not match the expected CID")]
    IncorrectCid,
    #[error("requested data not found")]
    NotFound,
}

/// Wrapper for IPFS's storage needs.
#[derive(Clone)]
pub(crate) struct Repository {
    inner: Arc<RepoInner>,
}

struct RepoInner {
    /// Raw key-value store for blocks.
    block_store: Box<dyn BlockStore>,
    /// Tracks pinning of blocks.
    pin_store: Box<dyn PinStore>,
    /// Key management store
    key_store: Box<dyn KeyStore>,
}

impl Repository {
    pub fn new(block_store: impl BlockStore + 'static, pin_store: impl PinStore + 'static, key_store: impl KeyStore + 'static) -> Self {
        Self { 
            inner: Arc::new(RepoInner {
                block_store: Box::new(block_store),
                pin_store: Box::new(pin_store),
                key_store: Box::new(key_store),
            })
        }
    }

    /// Graceful shutdown
    pub fn shutdown(&self) {
        todo!()
    }
    
    pub async fn contains(&self, cid: &Cid) -> Result<bool, RepoError> {
        self.inner.block_store.contains(cid).await
    }

    pub async fn get_block(&self, cid: &Cid) -> Result<Block, RepoError> {
        self.inner.block_store.get(cid).await
    }

    pub async fn put_block(&self, block: Block, pin_mode: PinMode) -> Result<(), RepoError> {
        let cid = block.cid().clone();
        self.inner.block_store
            .put(block)
            .await?;
        self.inner.pin_store
            .pin(&cid, pin_mode)
            .await?;
        Ok(())
    }

    pub async fn remove_block(&self, cid: &Cid) {
        todo!()
    }

    pub async fn list_blocks(&self) {
        todo!()
    }
}

