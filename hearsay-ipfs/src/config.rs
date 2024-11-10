use std::sync::Arc;

use libp2p::{futures::{channel::mpsc, select, StreamExt}, identity::Keypair, kad, swarm::NetworkBehaviour, Multiaddr, StreamProtocol};

use crate::{p2p::create_swarm, repo::{self, RepoInner, Repository}, task::IpfsTask, Ipfs};

/// Uninitiallized IPFS configuration.
pub struct IpfsConfig<C> 
where 
    C: NetworkBehaviour,
{
    pub keypair: Keypair,
    /// nodes to bootstrap from
    pub bootstrap: Vec<Multiaddr>,
    pub repo_config: repo::Config,
    pub kad_config: kad::Config,
    /// custom [NetworkBehaviour] 
    pub custom: Option<C>
}

impl<C> IpfsConfig<C> 
where 
    C: NetworkBehaviour,
{
    pub fn new(keypair: Keypair) -> Self {
        Self {
            keypair,
            bootstrap: vec![],
            repo_config: repo::Config::default(),
            kad_config: kad::Config::new(StreamProtocol::new("/test")), // TODO: change protocol name
            custom: None,
        }
    }

    /// Spawns IPFS background task.
    /// Returns [Ipfs] facade. 
    pub async fn start(self) -> Result<Ipfs<C>, Box<dyn std::error::Error>> {
        let repo = Repository { inner : Arc::new(RepoInner { capacity: 0.into() })}; // TODO: here

        let swarm = create_swarm(self).await?;

        let (task_tx, mut task_rx) = mpsc::channel::<IpfsTask>(0);

        crate::spawn(async move { 
            loop {
            select! { // TODO: make agnostic
                task = task_rx.select_next_some() => { task.handle() },
                //_ = cancel_token.cancelled() => { break; },
            }
        }});

        Ok(Ipfs {
            repo: repo.clone(),
            swarm,
            task_tx,
        })
    }
}
