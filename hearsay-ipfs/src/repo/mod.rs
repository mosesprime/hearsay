//! IPFS repository implementation

use std::sync::{atomic::AtomicUsize, Arc};

use blockstore::BlockStore;
use datastore::DataStore;

mod blockstore;
mod datastore;
mod config;
mod keystore;
pub use config::Config;
use keystore::KeyStore;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    // TODO: add errors here
}

#[derive(Clone)]
pub struct Repository {
    pub(crate) inner: Arc<RepoInner>,
}

pub(crate) struct RepoInner {
    capacity: AtomicUsize,
    data_store: Box<dyn DataStore>,
    block_store: Box<dyn BlockStore>,
    key_store: Box<dyn KeyStore>,
}

impl Repository {
    /// Graceful shutdown
    pub fn shutdown(&self) {
        todo!()
    }
}

