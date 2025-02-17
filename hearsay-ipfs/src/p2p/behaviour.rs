use std::fmt::Debug;

use libp2p::{identity::Keypair, kad, swarm::{behaviour::toggle::Toggle, NetworkBehaviour}, Multiaddr};

/// IPFS [NetworkBehaviour]
#[derive(NetworkBehaviour)]
pub(crate) struct IpfsBehaviour<C>
where 
    C: NetworkBehaviour,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send, 
{
    pub kademlia: Toggle<kad::Behaviour<kad::store::MemoryStore>>,
    pub custom: Toggle<C>,
}

impl<C> IpfsBehaviour<C>
where 
    C: NetworkBehaviour,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    pub(crate) fn new(
        keypair: &Keypair,
        kad_config: kad::Config,
        bootstrap: Vec<Multiaddr>,
        custom: Option<C>,
    ) -> Result<Self, ()> { // TODO: impl error
        let local_id = keypair.public().to_peer_id();

        let store = kad::store::MemoryStore::new(local_id);
        let kademlia = kad::Behaviour::with_config(local_id, store, kad_config);

        let behaviour = IpfsBehaviour {
            kademlia: Toggle::from(Some(kademlia)),
            custom: Toggle::from(custom),
        };
    
        for addr in bootstrap {
            todo!()
        }

        Ok(behaviour)
    }
}
