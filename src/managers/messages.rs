use crate::database::{MysqlConn, MysqlDb, RedisDb};
use crate::managers::connections::SenderType;
use crate::managers::peer_manager::PeerManager;
use crate::models::{
    Message, MessageErrors, MessageSuccess, MessageTags, NewUser, Peer, PeerStatus, ServerMessage,
    User,
};
use crate::utils::CRUD;
use chrono::{NaiveDateTime, Utc};
use std::net::SocketAddr;

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
    /*Routing Message Tags*/
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
        let peer_manager_clone = peer_manager.clone();
        match msg.tag {
            MessageTags::Login => {
                response_msg.tag = MessageTags::Login;
                let data_txt = msg.data.replace("\\", "");
                /*
                {"tag":"Login","data":"{\"username\":\"admin\",\"password\":\"123456\"}"}
                */
                let data_login = NewUser::parser_from_str(data_txt.as_str());
                if data_login.is_some() {
                    let user_form = data_login.unwrap();
                    let current_peer =
                        peer_manager_clone.find_by_username(user_form.username.clone().as_str());
                    if current_peer.is_none() {
                        if user_form.validate() {
                            let user_data = User::login(&user_form, &db, redis);
                            if user_data.is_some() {
                                let user = user_data.unwrap();
                                let old_peer = peer_manager_clone.find_by_id(user.id.clone());
                                /*Logic Reconnect*/
                                if old_peer.is_none() {
                                    let peer = Peer::new(
                                        owner_addr.clone(),
                                        user.clone(),
                                        true,
                                        PeerStatus::Lobby,
                                    );
                                    peer_manager.add_peer(peer);
                                } else {
                                    let mut peer = old_peer.unwrap();
                                    let peer_update = peer.update_data(
                                        owner_addr.clone(),
                                        user.clone(),
                                        true,
                                        PeerStatus::Lobby,
                                        Utc::now().naive_utc(),
                                        peer.room_id.clone(),
                                    );
                                    peer_manager.update_peer(&peer_update);
                                }
                                response_msg.status = true;
                                response_msg.message = Some(MessageSuccess::Login.to_string());
                                response_msg.data = Some(user.to_string());
                                return sender
                                    .send(owner_addr.clone(), response_msg.to_vec_u8().as_ref());
                            }
                            response_msg.message = Some(MessageErrors::UsnOrPwdInvalid.to_string())
                        }
                    }
                    let last_peer_data = current_peer.unwrap();
                    if last_peer_data.is_login {
                        response_msg.message = Some(MessageErrors::Logged.to_string());
                    }
                }
            }
            MessageTags::Register => {
                response_msg.tag = MessageTags::Register;
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
                            response_msg.message = Some(MessageSuccess::Register.to_string());
                            response_msg.data = Some(user.to_string());
                            return sender
                                .send(owner_addr.clone(), response_msg.to_vec_u8().as_ref());
                        } else {
                            response_msg.message = Some(MessageErrors::UsernameExist.to_string());
                        }
                    } else {
                        response_msg.message = Some(MessageErrors::UsnOrPwdInvalid.to_string());
                    }
                }
            }
            _ => {}
        }
        sender.send(owner_addr.clone(), response_msg.to_vec_u8().as_ref())
    }
}
