#[derive(Clone, Debug)]
pub enum PeerStatus {
    None,
    Lobby,
    Room,
    Board,
    Playing,
    Disconnect,
}
