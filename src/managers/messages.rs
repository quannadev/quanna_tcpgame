use crate::database::{MysqlConn, MysqlDb, RedisDb};
use crate::managers::connections::SenderType;
use crate::managers::peer_manager::PeerManager;
use crate::models::{NewUser, Peer, PeerStatus, ServerMessage, User, CRUD};
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
        let usr = NewUser::new("quanna".to_string(), "1234456".to_string());
        Self {
            tag: MessageTags::Join,
            data: usr.as_string(),
        }
    }

    pub fn exit_msg(addr: &SocketAddr) -> Self {
        Self {
            tag: MessageTags::Exit,
            data: addr.to_string(),
        }
    }
}
#[derive(Clone)]
pub struct MessageManager {
    redis: RedisDb,
    db: MysqlDb,
}
impl MessageManager {
    pub fn new(redis: RedisDb, db: MysqlDb) -> Self {
        MessageManager { redis, db }
    }
    pub fn parser(&mut self, payload: &[u8]) -> Option<Message> {
        let txt = std::str::from_utf8(payload).unwrap();
        let msg = Message::parse_struct(&txt);
        match msg {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
    pub fn message_router(
        &mut self,
        msg: Message,
        owner_addr: &SocketAddr,
        peer_manager: &mut PeerManager,
        sender: &mut SenderType,
    ) {
        let db = self.db.clone().conn.get().unwrap();
        let redis = self.redis.clone();
        let mut response_msg = ServerMessage::default();
        let peer_old = peer_manager.find_peer_by_addr(&owner_addr.clone());
        match msg.tag {
            MessageTags::Login => {
                response_msg.tag = MessageTags::Login;
                if peer_old.is_some() {
                    let peer_data = peer_old.unwrap();
                } else {
                    let data_txt = msg.data.replace("\\", "");
                    //{"tag":"Login","data":"{\"username\":\"admin\",\"password\":\"123456\"}"}
                    let data_login = NewUser::parser_from_str(data_txt.as_str());
                    if data_login.is_some() {
                        let user_form = data_login.unwrap();
                        if user_form.validate() {
                            let user_data = User::find_by_name(user_form.username, &db, redis);
                            if user_data.is_ok() {
                                let user = user_data.unwrap();
                                let peer = Peer::new(
                                    owner_addr.clone(),
                                    user.clone(),
                                    true,
                                    PeerStatus::Lobby,
                                );
                                peer_manager.add_peer(peer);
                                response_msg.status = true;
                                response_msg.data = Some(user.to_string());
                                return sender
                                    .send(owner_addr.clone(), response_msg.to_vec_u8().as_ref());
                            }
                        }
                    }
                }
            }
            MessageTags::Register => {
                response_msg.tag = MessageTags::Login;
                if peer_old.is_some() {
                    let peer_data = peer_old.unwrap();
                } else {
                    let data_txt = msg.data.replace("\\", "");
                    //{"tag":"Register","data":"{\"username\":\"admin\",\"password\":\"123456\"}"}
                    let data_login = NewUser::parser_from_str(data_txt.as_str());
                    if data_login.is_some() {
                        let user_form = data_login.unwrap();
                        if user_form.validate() {
                            let user_data = User::insert(&user_form, &db, redis);
                            if user_data.is_ok() {
                                let user = user_data.unwrap();
                                let peer = Peer::new(
                                    owner_addr.clone(),
                                    user.clone(),
                                    true,
                                    PeerStatus::Lobby,
                                );
                                peer_manager.add_peer(peer);
                                response_msg.status = true;
                                response_msg.data = Some(user.to_string());
                                return sender
                                    .send(owner_addr.clone(), response_msg.to_vec_u8().as_ref());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        sender.send(owner_addr.clone(), response_msg.to_vec_u8().as_ref())
    }
}
