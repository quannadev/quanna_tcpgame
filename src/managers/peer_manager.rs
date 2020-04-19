use crate::models::Peer;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct PeerManager {
    pub list_peers: Box<HashMap<i32, Peer>>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            list_peers: Box::new(HashMap::new()),
        }
    }
    pub fn add_peer(&mut self, peer: Peer) {
        self.list_peers.insert(peer.id.clone(), peer);
    }
    pub fn remove_peer(&mut self, id: &i32) {
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
    pub fn find_by_id(&self, uid: i32) -> Option<Peer> {
        self.list_peers.get(&uid).cloned()
    }
    pub fn find_by_username(&self, usrname: &str) -> Option<&Peer> {
        let mut peerx = None;
        for list_peer in self.list_peers.iter() {
            let peer = list_peer.1;
            if peer.user_name.eq(usrname) {
                peerx = Some(peer);
                break;
            }
        }
        peerx
    }
    pub fn update_peer(&mut self, peer: &Peer) {
        let peer_x = self
            .list_peers
            .entry(peer.id.clone())
            .or_insert(peer.clone());
        *peer_x = peer.clone();
    }
}
