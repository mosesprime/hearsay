use std::fmt::Debug;

use libp2p::{futures::channel::mpsc, identity::Keypair, kad, swarm::NetworkBehaviour, Multiaddr, StreamProtocol};
use tokio_util::sync::CancellationToken;

use crate::{p2p::create_swarm, repo::Repository, task::{IpfsHandler, IpfsTask}, Ipfs};

/// Uninitiallized IPFS configuration.
pub struct IpfsConfig<C> 
where 
    C: NetworkBehaviour,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    pub keypair: Keypair,
    /// nodes to bootstrap from
    pub bootstrap: Vec<Multiaddr>,
    pub kad_config: kad::Config,
    /// custom [NetworkBehaviour] 
    pub custom: Option<C>
}

impl<C> IpfsConfig<C> 
where 
    C: NetworkBehaviour + Send,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    pub fn new(keypair: Keypair) -> Self {
        Self {
            keypair,
            bootstrap: vec![],
            kad_config: kad::Config::new(StreamProtocol::new("/test")), // TODO: change protocol name
            custom: None,
        }
    }

    /// Spawns IPFS background task.
    /// Returns [Ipfs] facade. 
    pub async fn start(self) -> Result<Ipfs, Box<dyn std::error::Error>> {
        let repo: Repository = todo!(); // TODO: init repo

        let swarm = create_swarm(self).await?;

        let (task_tx, task_rx) = mpsc::channel::<IpfsTask>(0);
        let cancel_token = CancellationToken::new();

        let mut ipfs_handler = IpfsHandler::new(repo.clone(), swarm, cancel_token.clone(), task_rx);
        
        crate::spawn(async move { 
            ipfs_handler.run().await; // TODO: need this await?
        });

        Ok(Ipfs {
            repo,
            cancel_token,
            task_tx,
        })
    }
}
