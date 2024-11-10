use libp2p::{futures::channel::oneshot, swarm::dial_opts::DialOpts, PeerId};

pub(crate) enum IpfsTask {
    /// Establish a new connection to peer.
    Connect(DialOpts, oneshot::Sender<()>),
    /// Check connection to peer.
    IsConnected(PeerId, oneshot::Sender<bool>),
}

impl IpfsTask {
    pub(crate) fn handle(&self) {
        use IpfsTask::*;
        match self {
            Connect(dial_opts, tx) => {
                todo!()
            },
            IsConnected(peer_id, tx) => todo!(),
        }
    }
}
