// use crate::managers::connections::SenderType;
// use amethyst::ecs::Write;
// use amethyst::network::simulation::TransportResource;
use serde_json::Error as SerdeError;
use std::net::SocketAddr;

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
    pub fn parse_struct(txt: &str) -> Result<Self, SerdeError> {
        let msg: Message = serde_json::from_str(txt)?;
        Ok(msg)
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
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
}

#[derive(Default)]
pub struct MessageManager;
impl MessageManager {
    pub fn parser(&mut self, payload: &[u8]) -> Option<Message> {
        let txt = std::str::from_utf8(payload).unwrap();
        let raw_msg = txt.replace("\\", "");
        let msg = Message::parse_struct(&raw_msg);

        match msg {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}
