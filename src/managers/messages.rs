use amethyst::ecs::Write;
use amethyst::network::simulation::TransportResource;
use serde_json::Error as SerdeError;
// use amethyst::Result as AmethystResult;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    message: String,
}

impl Message {
    pub fn parse_struct(txt: &str) -> Result<Self, SerdeError> {
        let msg: Message = serde_json::from_str(txt)?;
        Ok(msg)
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn response(&self) -> Vec<u8> {
        let message = format!("confirm on receipt: {}", self.message);
        let msg = Message { message };
        serde_json::to_vec(&msg).unwrap()
    }
}

pub struct MessageManager<'a> {
    pub sender: &'a Write<'a, TransportResource>,
}

impl<'a> MessageManager<'a> {
    pub fn init(sender: &'a Write<'a, TransportResource>) -> Self {
        MessageManager { sender }
    }

    pub fn parser(&self, payload: &[u8]) -> Option<Message> {
        let txt = std::str::from_utf8(payload).unwrap();
        let raw_msg = txt.replace("\\", "");
        let msg = Message::parse_struct(&raw_msg);

        match msg {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}
