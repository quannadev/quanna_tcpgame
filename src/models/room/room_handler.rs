use crate::models::room::{Room, RoomErrorCodes, RoomStatus, RoomTypes};
use crate::models::Peer;
use std::collections::HashMap;
type ResultRoom<T> = std::result::Result<T, RoomErrorCodes>;
impl Room {
    pub fn new(
        id: i32,
        name: String,
        owner: i32,
        game_name: String,
        status: RoomStatus,
        room_type: RoomTypes,
        is_look: bool,
        password: Option<String>,
    ) -> Self {
        Room {
            id,
            name,
            owner,
            max_peer: 10,
            game_name,
            status,
            is_full: false,
            is_look,
            password,
            list_peers: Box::new(HashMap::new()),
            room_type,
        }
    }
    pub fn add_peer(&mut self, peer: Peer) -> ResultRoom<bool> {
        if self.list_peers.contains_key(&peer.id) {
            return Err(RoomErrorCodes::PeerExist);
        }
        self.list_peers.insert(peer.id.clone(), peer);
        Ok(true)
    }
    pub fn remove_peer_id(&mut self, id: i32) -> ResultRoom<bool> {
        if self.list_peers.contains_key(&id) {
            self.list_peers.remove(&id);
            return Ok(true);
        }
        Err(RoomErrorCodes::PeerExist)
    }
}
