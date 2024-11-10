pub(crate) use behaviour::IpfsBehaviour;
use libp2p::{swarm::NetworkBehaviour, Swarm};
use std::fmt::Debug;
use crate::config::IpfsConfig;

mod behaviour;
pub(crate) use behaviour::IpfsBehaviourEvent;

/// Utility to create a new [Swarm] for non-wasm.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn create_swarm<C>(
    ipfs_config: IpfsConfig<C>,
) -> Result<Swarm<IpfsBehaviour<C>>, Box<dyn std::error::Error>>
where 
    C: NetworkBehaviour,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    let keypair = ipfs_config.keypair;
    let peer_id = keypair.public().to_peer_id();

    let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_quic()
        .with_dns()?
        .with_behaviour(|keypair| IpfsBehaviour::new(
                keypair,
                ipfs_config.kad_config,
                ipfs_config.bootstrap,
                ipfs_config.custom,
        ).unwrap())? // TODO: handle err
        .with_swarm_config(|cfg| cfg)
        .build();

    Ok(swarm)
}

/// Utility to create a new [Swarm] for wasm.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn create_swarm<C>(
    ipfs_config: IpfsConfig<C>,
) -> Result<Swarm<IpfsBehaviour<C>>, Box<dyn std::error::Error>>
where 
    C: NetworkBehaviour,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    let keypair = ipfs_config.keypair;
    let peer_id = keypair.public().to_peer_id();

    let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_wasm_bindgen()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|keypair| IpfsBehaviour::new(
                keypair,
                ipfs_config.kad_config,
                ipfs_config.bootstrap,
                ipfs_config.custom,
        ).unwrap())? // TODO: handle err
        .with_swarm_config(|cfg| cfg)
        .build();

    Ok(swarm)
}


