use crate::device::peer::AllowedIP;

pub trait Registry {
    /// Returns a new peer candidate as defined by the implementing registry
    fn new_candidate(&self, _public_key: &x25519_dalek::PublicKey) -> Option<PeerCandidate> {
        None
    }

    /// Register the candidate typically resulting in the candidate becoming a full peer.
    fn register_candidate(&self, _candidate: PeerCandidate) {
        unimplemented!()
    }
}

pub struct PeerCandidate {
    pub public_key: x25519_dalek::PublicKey,
    pub allowed_ips: Vec<AllowedIP>,
    pub keepalive: u16,
}

#[derive(Default)]
pub struct NopRegistry;

impl Registry for NopRegistry {}
