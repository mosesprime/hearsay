mod block;
mod config;
mod ipns;
mod p2p;
mod repo;
mod task;

use p2p::IpfsBehaviour;
use repo::Repository;
use task::IpfsTask;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use tokio::task::spawn;
use tokio_util::sync::CancellationToken;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm_bindgen_futures::spawn_local as spawn;

pub use block::Block;
use libp2p::{futures::{channel::{mpsc, oneshot}, SinkExt}, identity::Keypair, swarm::{dial_opts::DialOpts, NetworkBehaviour}, PeerId, StreamProtocol, Swarm};


/// IPFS node, built from [config::IpfsConfig].
pub struct Ipfs {
    pub(crate) repo: Repository,
    pub(crate) cancel_token: CancellationToken,
    pub(crate) task_tx: mpsc::Sender<IpfsTask>
}


impl Ipfs {
    /// Establish a new connection to peer.
    pub async fn connect(&self, dial_opts: DialOpts) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = oneshot::channel();
        self.task_tx
            .clone()
            .send(IpfsTask::Connect(dial_opts, tx))
            .await?;
        Ok(rx.await?)
    }

    /// Check connection to peer.
    pub async fn is_connected(&self, peer_id: PeerId) -> Result<bool, Box<dyn std::error::Error>> {
        let (tx, rx) = oneshot::channel();
        self.task_tx
            .clone()
            .send(IpfsTask::IsConnected(peer_id, tx))
            .await?;
        Ok(rx.await?)
    }
}

#[tokio::test]
async fn test_ipfs_start() {
    use config::IpfsConfig;
    let _ = IpfsConfig::<libp2p::swarm::dummy::Behaviour> {
        keypair: Keypair::generate_ed25519(),
        bootstrap: vec![],
        repo_config: repo::Config::default(),
        kad_config: libp2p::kad::Config::new(StreamProtocol::new("/test")),
        custom: None,
    }.start().await.unwrap();
}
