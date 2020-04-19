#[derive(Clone)]
pub enum RoomErrorCodes {
    Full,
    PeerExist,
    PeerNotFound,
}
impl RoomErrorCodes {
    pub fn to_string(&self) -> String {
        match self {
            RoomErrorCodes::Full => format!("Room is full"),
            RoomErrorCodes::PeerExist => format!("Peer exist"),
            RoomErrorCodes::PeerNotFound => format!("Peer not found"),
        }
    }
}
