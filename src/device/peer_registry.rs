use std::net::IpAddr;
use crate::crypto::X25519PublicKey;
use crate::device::peer::AllowedIP;

// A Peer that is not yet a Peer
pub struct PrePeer {
    pub public_key: X25519PublicKey,
    pub allowed_ips: Vec<AllowedIP>,
    pub keepalive: u16
}

pub trait PeerRegistry: 'static + Send + Sync {
    // Get a peer from the registry with the indicated public key
    fn get(&self, public_key: X25519PublicKey) -> Option<PrePeer>;

    // Add a route for the indicated IP and CIDR
    fn add_route(&self, ip: &IpAddr, cidr: &u8);

    // Update a peer's IP in the registry
    fn update_ip(&self, public_key: X25519PublicKey, ip: &IpAddr);
}

pub struct EmptyPeerRegistry {}

// This PeerRegistry impl is intended to do nothing
impl PeerRegistry for EmptyPeerRegistry {
    fn get(&self, _public_key: X25519PublicKey) -> Option<PrePeer> {
        None
    }

    fn add_route(&self, _ip: &IpAddr, _cidr: &u8) {}

    fn update_ip(&self, _public_key: X25519PublicKey, _ip: &IpAddr) {}
}