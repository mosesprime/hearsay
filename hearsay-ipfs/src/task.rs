use std::fmt::Debug;

use libp2p::{futures::{channel::{mpsc, oneshot}, select, FutureExt, StreamExt}, swarm::{dial_opts::DialOpts, NetworkBehaviour, SwarmEvent}, PeerId, Swarm, kad};
use tokio_util::sync::CancellationToken;
use tracing::{trace, warn};

use crate::{p2p::{IpfsBehaviour, IpfsBehaviourEvent}, repo::Repository};

pub(crate) enum IpfsTask {
    /// Establish a new connection to peer.
    Connect(DialOpts, oneshot::Sender<()>),
    /// Check connection to peer.
    IsConnected(PeerId, oneshot::Sender<bool>),
}

/// IPFS background task handler.
pub(crate) struct IpfsHandler<C>
where 
    C: NetworkBehaviour + Send,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    repo: Repository,
    swarm: Swarm<IpfsBehaviour<C>>,
    cancel_token: CancellationToken,
    task_rx: mpsc::Receiver<IpfsTask>,
}

impl<C> IpfsHandler<C> 
where 
    C: NetworkBehaviour + Send,
    <C as NetworkBehaviour>::ToSwarm: Debug + Send,
{
    pub(crate) fn new(repo: Repository, swarm: Swarm<IpfsBehaviour<C>>, cancel_token: CancellationToken, task_rx: mpsc::Receiver<IpfsTask>) -> Self {
        Self {
            repo,
            swarm,
            cancel_token,
            task_rx,
        }
    }

    pub(crate) async fn run(&mut self) {
        loop {
            select! {
                event = self.swarm.select_next_some() => self.handle_swarm_event(event),
                task = self.task_rx.select_next_some() => self.handle_ipfs_task(task),
                _ = self.cancel_token.cancelled().fuse() => {
                    self.repo.shutdown();
                    // TODO: graceful shutdown
                    break;
                },
            }
        }
    }
    
    fn handle_ipfs_task(&mut self, task: IpfsTask) {
        use IpfsTask::*;
        match task {
            Connect(dial_opts, tx) => todo!(),
            IsConnected(peer_id, tx) => todo!(),
        }
    }

    fn handle_swarm_event(&mut self, event: SwarmEvent<<IpfsBehaviour<C> as NetworkBehaviour>::ToSwarm>) {
        use SwarmEvent::*;
        use kad::Event::*;
        match event {
            ConnectionEstablished { peer_id, connection_id, endpoint, num_established, concurrent_dial_errors, established_in } => todo!(),
            ConnectionClosed { peer_id, connection_id, endpoint, num_established, cause } => todo!(),
            IncomingConnection { connection_id, local_addr, send_back_addr } => todo!(),
            IncomingConnectionError { connection_id, local_addr, send_back_addr, error } => todo!(),
            OutgoingConnectionError { connection_id, peer_id, error } => todo!(),
            NewListenAddr { listener_id, address } => {
                tracing::trace!("new listen addr: {listener_id} at {address}");
                todo!()
            },
            ExpiredListenAddr { listener_id, address } => todo!(),
            ListenerClosed { listener_id, addresses, reason } => todo!(),
            ListenerError { listener_id, error } => todo!(),
            Dialing { peer_id, connection_id } => todo!(),
            NewExternalAddrCandidate { address } => todo!(),
            ExternalAddrConfirmed { address } => todo!(),
            ExternalAddrExpired { address } => todo!(),
            NewExternalAddrOfPeer { peer_id, address } => todo!(),
            Behaviour(IpfsBehaviourEvent::Kademlia(e)) => match e {
                InboundRequest { request } => {
                    trace!("kademlia: inbound request {:?}", request);
                },
                OutboundQueryProgressed { id, result, stats, step } => todo!(),
                RoutingUpdated { peer, addresses, .. } => {
                    trace!("kademlia: routing updated {} {:?}", peer, addresses);
                },
                UnroutablePeer { peer } => {
                    trace!("kademlia: unroutable peer {}", peer);
                },
                RoutablePeer { peer, address } => {
                    trace!("kademlia: routable peer {} {}", peer, address);
                },
                PendingRoutablePeer { peer, address } => {
                    trace!("kademlia: pending routable peer {} {}", peer, address);
                },
                ModeChanged { new_mode } => {
                    trace!("kademlia: mode changed {}", new_mode);
                },
            },
            Behaviour(IpfsBehaviourEvent::Custom(event)) => {
                warn!("unhandled custom event: {:?}", event)
            }
            event => {
                warn!("unhandled swarm event: {:?}", event)
            }
        }
    }
}
