use crate::managers::connections::SenderType;
use crate::models::{PeerStatus, User};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Peer {
    pub id: String,
    pub addr: SocketAddr,
    pub is_login: bool,
    pub status: PeerStatus,
    pub user: User,
}
impl Peer {
    pub fn new(addr: SocketAddr, user: User, is_login: bool, status: PeerStatus) -> Self {
        Self {
            id: Peer::gen_id(),
            addr,
            user,
            is_login,
            status,
        }
    }
    fn gen_id() -> String {
        let mut rgn = thread_rng();
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
        (0..10)
            .map(|_| {
                let idx = rgn.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
