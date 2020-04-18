use crate::database::{MysqlDb, RedisDb};
use crate::managers::messages::{Message, MessageManager};
use amethyst::ecs::Write;
use amethyst::network::simulation::TransportResource;
// use std::collections::HashMap;
use std::net::SocketAddr;

pub type SenderType<'a> = Write<'a, TransportResource>;

#[derive(Clone)]
pub struct ConnectionManager {
    pub list_conn: Vec<SocketAddr>,
    pub redis: RedisDb,
    pub mysql: MysqlDb,
}

impl ConnectionManager {
    pub fn init(redis: RedisDb, mysql: MysqlDb) -> Self {
        ConnectionManager {
            list_conn: Vec::new(),
            redis,
            mysql,
        }
    }

    pub fn on_connect<'a>(&mut self, addr: &SocketAddr, sender: &mut SenderType) {
        self.list_conn.push(*addr);
        let msg = format!("New connect {}\r\n", &addr);
        info!("{}", msg);
        self.send_without_me(sender, addr, Message::join_msg(&addr))
    }

    pub fn on_disconnect<'a>(&mut self, addr: &SocketAddr, sender: &mut SenderType) {
        let idx = self.list_conn.iter().position(|a| a == addr);

        if idx.is_some() {
            self.list_conn.remove(idx.unwrap());
            let msg = format!("Client {} disconnected \r\n", &addr);
            info!("{}", msg);
            self.send_without_me(sender, addr, Message::exit_msg(&addr));
        }

        info!("Count socket {}", self.list_conn.len())
    }

    pub fn on_message<'a>(&mut self, addr: SocketAddr, payload: &[u8], sender: &mut SenderType) {
        let message = MessageManager::default().parser(payload);
        if message.is_some() {
            let msg_pared = message.unwrap();
            sender.send(addr, msg_pared.to_vec_u8().as_ref())
        }
    }

    pub fn send_all<'a>(&mut self, sender: &mut SenderType, payload: Message) {
        let send_data = |s: &SocketAddr| Self::send_message(*s, payload.clone(), sender);
        self.list_conn.iter().for_each(send_data);
    }

    pub fn send_without_me<'a>(
        &self,
        sender: &mut Write<'a, TransportResource>,
        me: &SocketAddr,
        payload: Message,
    ) {
        for socket in self.list_conn.iter() {
            if socket.eq(&me) {
                continue;
            }
            Self::send_message(*socket, payload.clone(), sender)
        }
    }

    pub fn send_message(
        socket: SocketAddr,
        payload: Message,
        sender: &mut Write<TransportResource>,
    ) {
        sender.send(socket, payload.to_vec_u8().as_ref())
    }
}
