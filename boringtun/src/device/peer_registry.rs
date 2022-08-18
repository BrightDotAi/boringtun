use std::net::IpAddr;
use crate::device::peer::AllowedIP;

// A Peer that is not yet a Peer
pub struct PrePeer {
    pub public_key: x25519_dalek::PublicKey,
    pub allowed_ips: Vec<AllowedIP>,
    pub keepalive: u16
}

pub trait PeerRegistry: 'static + Send + Sync {
    // Get a peer from the registry with the indicated public key
    fn get(&self, public_key: x25519_dalek::PublicKey) -> Option<PrePeer>;

    // Add a route for the indicated IP and CIDR
    fn add_route(&self, ip: &IpAddr, cidr: &u8);

    // Update a peer in the registry
    fn update(&self, public_key: x25519_dalek::PublicKey);
}

pub struct EmptyPeerRegistry {}

// This PeerRegistry impl is intended to do nothing
impl PeerRegistry for EmptyPeerRegistry {
    fn get(&self, _public_key: x25519_dalek::PublicKey) -> Option<PrePeer> {
        None
    }

    fn add_route(&self, _ip: &IpAddr, _cidr: &u8) {}

    fn update(&self, _public_key: x25519_dalek::PublicKey) {}
}
