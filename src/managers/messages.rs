use amethyst::{ecs::Write, network::simulation::TransportResource, Result};
pub struct MessageManager<'a> {
    pub sender: &'a Write<'a, TransportResource>,
}
impl<'a> MessageManager<'a> {
    pub fn init(sender: &'a Write<'a, TransportResource>) -> Self {
        MessageManager { sender }
    }
    pub fn parser(&self, payload: &[u8]) -> Option<Message> {
        let txt = std::str::from_utf8(payload).unwrap();
        let msg = Message::parse_struct(txt);
        if msg.is_some() {
            return Some(msg.unwrap());
        }
        None
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    message: String,
}

impl Message {
    pub fn parse_struct(txt: &str) -> Option<Self> {
        match serde_json::from_str::<Self>(txt) {
            Ok(message) => Some(message),
            _ => None,
        }
    }
    pub fn to_vec_u8(&self) -> Option<Vec<u8>> {
        match serde_json::to_vec(self) {
            Ok(value) => Some(value),
            _ => None,
        }
    }
}
