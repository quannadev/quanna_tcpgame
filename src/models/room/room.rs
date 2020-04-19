use crate::models::room::room_type::RoomTypes;
use crate::models::room::status::RoomStatus;
use crate::models::Peer;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub owner: i32,
    pub max_peer: usize,
    pub game_name: String,
    pub status: RoomStatus,
    pub is_full: bool,
    pub is_look: bool,
    pub password: Option<String>,
    pub list_peers: Box<HashMap<i32, Peer>>,
    pub room_type: RoomTypes,
}
