use std::{collections::{btree_map::Entry, BTreeMap}, mem::MaybeUninit, ptr::write_volatile};

use async_trait::async_trait;
use libp2p::futures::stream::{self, BoxStream};
use tokio::sync::RwLock;
use tracing::trace;

use crate::repo::RepoError;

use super::KeyStore;

/// In memory [KeyStore].
pub struct MemKeyStore {
    inner: RwLock<BTreeMap<String, Vec<u8>>>, // TODO: btree vs hash?
}

impl MemKeyStore {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMap::default()),
        }
    }
}

#[async_trait]
impl KeyStore for MemKeyStore {
    async fn contains(&self, domain: &str) -> Result<bool, RepoError> {
        Ok(self.inner.read().await.contains_key(domain))
    }

    async fn list(&self) -> Result<Vec<String>, RepoError> {
        let inner = &*self.inner.read().await;
        let keys = inner.clone().into_keys().collect();
        Ok(keys)
    }

    async fn get(&self, domain: &str) -> Result<Vec<u8>, RepoError> {
        let inner = &*self.inner.read().await;
        if let Some(data) = inner.get(domain) {
            return Ok(data.to_owned());
        }
        Err(RepoError::NotFound)
    }

    async fn put(&self, domain: &str, key: &[u8]) -> Result<(), RepoError> {
        let inner = &mut *self.inner.write().await;
        match inner.entry(domain.into()) {
            Entry::Occupied(mut oe) => {
                trace!("overwrite {} key entry", domain);
                *oe.get_mut() = key.into();
            },
            Entry::Vacant(ve) => {
                ve.insert(key.into());
            },
        }
        Ok(())
    }

    async fn remove(&self, domain: &str) -> Result<(), RepoError> {
        let inner = &mut *self.inner.write().await;
        match inner.remove(domain) {
            Some(mut k) => {
                // zero initialized elements
                k.iter_mut().for_each(|i| *i = 0);
                // drop elements and set len to 0
                k.clear();
                // zero space capacity
                k.spare_capacity_mut().iter_mut().for_each(|i| unsafe {
                    write_volatile(i, MaybeUninit::zeroed())
                });
                Ok(())
            },
            None => Err(RepoError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: add keystore tests
}
