use libp2p::dns::{ResolverConfig, ResolverOpts};
use thiserror::Error;

use crate::Ipfs;

impl Ipfs {
    pub async fn resolve_ipns(&self, path: &IpfsPath, recursive: bool) -> Result<IpfsPath, IpnsError> {
        todo!()
    }

    pub async fn resolve_dnslink(domain: &str, mut path: impl Iterator<Item = &str>) -> Result<IpfsPath, Error> {
        todo!()
    }

    pub async fn publish_ipns(&self, path: &IpfsPath) -> Result<IpfsPath, Error> {
        todo!()
    }
}
