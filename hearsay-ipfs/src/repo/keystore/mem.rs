use std::{collections::{btree_map::Entry, BTreeMap}, mem::MaybeUninit, ptr::write_volatile};

use async_trait::async_trait;
use libp2p::futures::stream::{self, BoxStream};
use tokio::sync::RwLock;
use tracing::trace;

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
    async fn contains(&self, domain: &str) -> Result<bool, Error> {
        Ok(self.inner.read().await.contains_key(domain))
    }

    async fn list(&self) -> Result<BoxStream<&str>, Error> {
        let inner = self.inner.read().await;
        stream::iter(inner.iter().map(|(d, _)| d.as_str()).collect::<Vec<_>>())
    }

    async fn get(&self, domain: &str) -> Result<Option<&[u8]>, Error> {
        Ok(self.inner.read().await.get(domain).map(|k| k.as_slice()))
    }

    async fn put(&self, domain: &str, key: &[u8]) -> Result<(), Error> {
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

    async fn remove(&self, domain: &str) -> Result<(), Error> {
        let inner = &mut *self.inner.write().await;
        match inner.remove(domain) {
            Some(k) => {
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
            None => Err(todo!()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: add keystore tests
}
