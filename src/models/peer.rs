use crate::managers::connections::SenderType;
use crate::models::{PeerStatus, User};
use chrono::{NaiveDateTime, Utc};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: String,
    pub addr: SocketAddr,
    pub is_login: bool,
    pub status: PeerStatus,
    pub user: User,
    pub last_login: NaiveDateTime,
}
impl Peer {
    pub fn new(addr: SocketAddr, user: User, is_login: bool, status: PeerStatus) -> Self {
        Self {
            id: Peer::gen_id(),
            addr,
            user,
            is_login,
            status,
            last_login: Utc::now().naive_utc(),
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
