use crate::managers::connections::SenderType;
use amethyst::{ecs::Write, network::simulation::TransportResource, Result};
use std::net::SocketAddr;

#[derive(Default)]
pub struct MessageManager;
impl MessageManager {
    pub fn parser(&mut self, payload: &[u8]) -> Option<Message> {
        let txt = std::str::from_utf8(payload).unwrap();
        let msg = txt.replace("\\", "");
        let msg = Message::parse_struct(msg.as_str());
        if msg.is_some() {
            let msg_parsed = msg.unwrap();
            return Some(msg_parsed);
        }
        warn!("message {} invalid", txt.replace("\\", ""));
        None
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageTags {
    None,
    Join,
    Exit,
    Config,
    Login,
    Register,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub tag: MessageTags,
    pub data: String,
}

impl Message {
    pub fn parse_struct(txt: &str) -> Option<Self> {
        match serde_json::from_str::<Self>(txt) {
            Ok(message) => Some(message),
            _ => None,
        }
    }
    pub fn join_msg(addr: &SocketAddr) -> Self {
        Self {
            tag: MessageTags::Join,
            data: addr.to_string(),
        }
    }
    pub fn exit_msg(addr: &SocketAddr) -> Self {
        Self {
            tag: MessageTags::Exit,
            data: addr.to_string(),
        }
    }
    pub fn to_vec_u8(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
