use std::collections::{hash_map::Entry, HashMap};

use async_trait::async_trait;
use cid::Cid;
use libp2p::futures::stream::{self, BoxStream};
use tokio::sync::RwLock;
use tracing::trace;

use super::{DataStore, PinStore};

/// In memory [DataStore] and [PinStore].
pub struct MemDataStore {
    inner: RwLock<MemDataStoreInner>,
}

struct MemDataStoreInner {
    data: HashMap<Vec<u8>, Vec<u8>>,
    pins: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemDataStore {
    pub fn new() -> Self {
        Self { 
            inner: RwLock::new(MemDataStoreInner {
                data: HashMap::default(),
                pins: HashMap::default(),
            }),
        }
    }
}

#[async_trait]
impl DataStore for MemDataStore {
    async fn contains(&self, key: &[u8]) -> Result<bool, Error> {
        Ok(self.inner.read().await.data.contains_key(key))
    }

    async fn list(&self) -> BoxStream<Result<&[u8], Error>> {
        let inner = &*self.inner.read().await;
        stream::iter(inner.data.keys().map(|x| x.into()).collect::<Vec<_>>())
    }

    async fn get(&self, key: &[u8]) -> Result<Option<&[u8]>, Error> {
        let inner = &*self.inner.read().await;
        if let Some(data) = inner.data.get(key) {
            return Ok(Some(data));
        }
        Ok(None)
    }

    async fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let inner = &mut *self.inner.write().await;
        match inner.data.entry(key.to_vec()) {
            Entry::Vacant(e) => {
                e.insert(value.to_vec());
            },
            Entry::Occupied(_) => {
                trace!("data {:?} already exists", key);
                todo!("handle mem datastore put already exists");
            },
        }
        Ok(())
    }

    async fn remove(&self, key: &[u8]) -> Result<(), Error> {
        let inner = &mut *self.inner.write().await;
        match inner.data.remove(key) {
            Some(_) => Ok(()),
            None => Err(todo!()),
        }
    }
}

#[async_trait]
impl PinStore for MemDataStore {
    async fn is_pinned(&self, cid: &Cid) -> Result<bool, Error> {
        let inner = &*self.inner.read().await;
        Ok(inner.pins.contains_key(&cid.to_bytes()))
    }

    async fn insert_pin(&self, cid: &Cid) -> Result<(), Error> {
        todo!()
    }

    async fn remove_pin(&self, cid: &Cid) -> Result<(), Error> {
        todo!()
    }

    async fn list_pins(&self) -> BoxStream<Result<Cid, Error>> {
        let inner = &*self.inner.read().await;
        stream::iter(inner.pins.iter().map(|(k, v)| {
            Cid::try_from(k.as_slice()) // TODO: map error
        }).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mem_datastore() {
        let store = MemDataStore::new();
        let key = b"banana";
        let value = b"banana";

        assert!(!store.contains(key).await.unwrap());
        assert!(store.get(key).await.unwrap() == None);
        store.remove(key).await.unwrap();
        store.put(key, value).await.unwrap();
        assert!(store.contains(key).await.unwrap());
        assert!(store.get(key).await.unwrap() == Some(value));
        store.remove(key).await.unwrap();
        assert!(!store.contains(key).await.unwrap());
    }

    // TODO: add mem datastore list test
    // TODO: add mem pinstore test
}
