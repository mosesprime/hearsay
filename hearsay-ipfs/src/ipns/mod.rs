use libp2p::dns::{ResolverConfig, ResolverOpts};
use thiserror::Error;

use crate::{path::IpfsPath, Ipfs};

impl Ipfs {
    pub async fn resolve_ipns(&self, path: &IpfsPath, recursive: bool) -> Result<IpfsPath, IpnsError> {
        todo!()
    }

    pub async fn resolve_dnslink(domain: &str, mut path: impl Iterator<Item = &str>) -> Result<IpfsPath, IpnsError> {
        todo!()
    }

    pub async fn publish_ipns(&self, path: &IpfsPath) -> Result<IpfsPath, IpnsError> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum IpnsError {} // TODO: impl ipns error
