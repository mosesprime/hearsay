use std::{fmt::Debug, hash::Hash};

use bytes::Bytes;
use cid::Cid;
use multihash_codetable::Code;
use multihash_derive::MultihashDigest;

#[derive(Clone, Eq)]
pub struct Block {
    cid: Cid,
    data: Bytes
}

impl Block {
    pub fn new(cid: Cid, data: Bytes) -> Result<Self, Error> {
        let block = Block { cid, data };
        if !block.verify() {
            return Err(todo!());
        }
        Ok(block)
    }

    pub fn cid(&self) -> &Cid {
        &self.cid
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn inner(&self) -> &Bytes {
        &self.data
    }

    pub fn verify(&self) -> bool {
        let hash = match Code::try_from(self.cid.hash().code()) {
            Ok(code) => code.digest(&self.data),
            Err(_) => return false,
        };
        hash.digest() == self.cid.hash().digest()
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("cid", &self.cid.hash())
            .field("data", &format!("{} bytes", self.data.len()))
            .finish()
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.cid.eq(&other.cid)
    }
}

impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self.cid, state)
    }
}
