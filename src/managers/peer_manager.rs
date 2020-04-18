use crate::models::Peer;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct PeerManager {
    pub list_peers: HashMap<String, Peer>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            list_peers: HashMap::new(),
        }
    }
    pub fn add_peer(&mut self, peer: Peer) {
        self.list_peers.insert(peer.id.clone(), peer);
    }
    pub fn remove_peer(&mut self, id: &str) {
        self.list_peers.remove(id);
    }
    pub fn find_peer_by_addr(&self, addr: &SocketAddr) -> Option<&Peer> {
        let mut peerx = None;
        for list_peer in self.list_peers.iter() {
            let peer = list_peer.1;
            if peer.addr.eq(addr) {
                peerx = Some(peer);
                break;
            }
        }
        peerx
    }
}
