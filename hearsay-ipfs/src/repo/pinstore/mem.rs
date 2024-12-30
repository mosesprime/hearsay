use std::collections::BTreeMap;
use async_trait::async_trait;
use cid::Cid;
use crate::repo::RepoError;
use super::{PinInfo, PinMode, PinStore};

struct MemPinStore {
    inner: tokio::sync::RwLock<BTreeMap<Cid, PinInfo>>,
}

#[async_trait]
impl PinStore for MemPinStore {
    async fn is_pinned(&self, cid: &Cid) -> Result<bool, RepoError> {
        todo!()
    }

    async fn pin(&self, cid: &Cid, mode: PinMode) -> Result<(), RepoError> {
        let inner = &mut *self.inner.write().await;
        inner.entry(*cid)
            .and_modify(|entry| match mode {
                PinMode::Direct => entry.direct = true,
                PinMode::Indirect(cid) => entry.indirect.push(cid), // TODO: ensure no duplicates
                PinMode::Recursive => entry.recursive += 1,
            }).or_insert(PinInfo { 
                direct: mode == PinMode::Direct,
                indirect: if let PinMode::Indirect(cid) = mode { vec![cid] } else { vec![] },
                recursive: if mode == PinMode::Recursive {1} else {0}
            });
        Ok(())
    }

    async fn unpin(&self, cid: &Cid) -> Result<(), RepoError> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<Cid>, RepoError> {
        todo!()
    }
}
