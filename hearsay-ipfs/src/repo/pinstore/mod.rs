use async_trait::async_trait;
use cid::Cid;

use super::RepoError;

mod mem;

/// Keeps track of which [Cid]s must remain pinned by the [Repo].
#[async_trait]
pub trait PinStore: Send + Sync {
    async fn is_pinned(&self, cid: &Cid) -> Result<bool, RepoError>;
    async fn pin(&self, cid: &Cid, mode: PinMode) -> Result<(), RepoError>;
    async fn unpin(&self, cid: &Cid) -> Result<(), RepoError>;
    async fn list(&self) -> Result<Vec<Cid>, RepoError>;
}


pub struct PinInfo {
    direct: bool,
    indirect: Vec<Cid>,
    recursive: u64,
}

#[derive(PartialEq)]
pub enum PinMode {
    Direct,
    Indirect(Cid),
    Recursive,
}
