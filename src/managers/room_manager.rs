use crate::models::{Room, RoomStatus, RoomTypes};
use std::collections::HashMap;

pub struct RoomsManager {
    pub current_id: i32,
    pub list_room: Box<HashMap<i32, Room>>,
}
impl RoomsManager {
    pub fn init() -> Self {
        RoomsManager {
            current_id: 0,
            list_room: Box::new(HashMap::new()),
        }
    }
    pub fn create_room(
        &mut self,
        name: String,
        owner: i32,
        game_name: String,
        status: RoomStatus,
        room_type: RoomTypes,
        is_look: bool,
        password: Option<String>,
    ) -> Room {
        let id = self.current_id + 1;
        let new_room = Room::new(
            id, name, owner, game_name, status, room_type, is_look, password,
        );
        self.add_room(new_room.clone());
        new_room
    }
    pub fn add_room(&mut self, room: Room) {
        if self.list_room.get(&room.id).is_none() {
            self.list_room.insert(room.id.clone(), room);
        }
    }
    pub fn remove_room(&mut self, id: i32) {
        if self.list_room.get(&id).is_some() {
            self.list_room.remove(&id);
        }
    }
    pub fn find_by_id(&self, id: i32) -> Option<&Room> {
        self.list_room.get(&id)
    }
}
