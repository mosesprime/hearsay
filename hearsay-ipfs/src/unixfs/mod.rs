//! UnixFS implimentation
//! See <https://github.com/ipfs/specs/blob/main/UNIXFS.md>

use std::{io::{self, Read, Write}, path::{Path, PathBuf}};

use libp2p::futures::{stream::BoxStream, Stream};
use thiserror::Error;

use crate::{path::IpfsPath, repo::Repository, Ipfs};

mod export;
mod import;
mod pb;

/// Wrapper for the IPFS [Repository] that handles file and stream I/O.
pub(crate) struct UnixFs {
    ipfs: Ipfs
}

impl UnixFs {
    pub fn new(ipfs: Ipfs) -> Self {
        Self { ipfs }
    }

    pub fn add<R: Read>(&self, stream: R) -> BoxStream<UnixFsStatus> {
        todo!()
    }

    pub fn get<W: Write>(&self, src: IpfsPath, dest: W) -> BoxStream<UnixFsStatus> {
        todo!()
    }
}

#[derive(Debug)]
pub enum UnixFsStatus {
    Progress,
    Complete,
    Error(UnixFsError),
}

#[derive(Debug, Error)]
pub enum UnixFsError {
    #[error(transparent)]
    Io(#[from] io::Error),
}
