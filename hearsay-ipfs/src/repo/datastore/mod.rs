use async_trait::async_trait;
use cid::Cid;
use libp2p::futures::stream::BoxStream;

mod mem;

#[async_trait]
pub trait DataStore: PinStore + Send + Sync {
    async fn contains(&self, key: &[u8]) -> Result<bool, Error>;
    async fn list(&self) -> BoxStream<Result<&[u8], Error>>; // TODO: Result<Stream<T>> instead?
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
