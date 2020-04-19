use crate::models::MessageTags;
use serde_json::Error as SerdeError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerMessage {
    pub status: bool,
    pub tag: MessageTags,
    pub message: Option<String>,
    pub data: Option<String>,
}
impl Default for ServerMessage {
    fn default() -> Self {
        Self {
            status: false,
            tag: MessageTags::None,
            message: None,
            data: None,
        }
    }
}
impl ServerMessage {
    pub fn new(
        status: bool,
        tag: MessageTags,
        message: Option<String>,
        data: Option<String>,
    ) -> Self {
        Self {
            status,
            tag,
            message,
            data,
        }
    }
    pub fn parse_struct(txt: &str) -> Result<Self, SerdeError> {
        let msg: ServerMessage = serde_json::from_str(txt)?;
        Ok(msg)
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
