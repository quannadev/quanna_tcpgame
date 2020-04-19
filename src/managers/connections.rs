use crate::database::{MysqlDb, RedisDb};
use crate::managers::messages::MessageManager;
use crate::managers::PeerManager;
use crate::models::Message;
use amethyst::ecs::Write;
use amethyst::network::simulation::TransportResource;
use std::net::SocketAddr;

pub type SenderType<'a> = Write<'a, TransportResource>;

#[derive(Clone)]
pub struct ConnectionManager {
    pub list_conn: Vec<SocketAddr>,
    pub redis: RedisDb,
    pub mysql: MysqlDb,
    pub peer_manager: PeerManager,
}

impl ConnectionManager {
    pub fn init(redis: RedisDb, mysql: MysqlDb) -> Self {
        ConnectionManager {
            list_conn: Vec::new(),
            redis,
            mysql,
            peer_manager: PeerManager::new(),
        }
    }

    pub fn on_connect(&mut self, addr: &SocketAddr, sender: &mut Write<TransportResource>) {
        self.list_conn.push(*addr);
        let msg = format!("New connect {}\r\n", &addr);
        info!("{}", msg);
        self.send_without_me(sender, addr, Message::join_msg(&addr))
    }

    pub fn on_disconnect(&mut self, addr: &SocketAddr, sender: &mut Write<TransportResource>) {
        let idx = self.list_conn.iter().position(|a| a == addr);

        if idx.is_some() {
            self.list_conn.remove(idx.unwrap());
            let msg = format!("Client {} disconnected \r\n", &addr);
            info!("{}", msg);
            self.send_without_me(sender, addr, Message::exit_msg(&addr));
        }

        info!("Count socket {}", self.list_conn.len());
        info!("Count peer {}", self.peer_manager.list_peers.len());
    }

    pub fn on_message(
        &mut self,
        addr: SocketAddr,
        payload: &[u8],
        sender: &mut Write<TransportResource>,
    ) {
        let mut message_manager = MessageManager::new(self.redis.clone(), self.mysql.clone());
        let message = message_manager.parser(payload);
        if message.is_some() {
            let msg_pared = message.unwrap();
            return message_manager.message_router(
                msg_pared,
                &addr,
                &mut self.peer_manager,
                sender,
            );
        }
    }

    pub fn send_all(&mut self, sender: &mut Write<TransportResource>, payload: Message) {
        let send_data = |s: &SocketAddr| Self::send_message(*s, payload.clone(), sender);
        self.list_conn.iter().for_each(send_data);
    }

    pub fn send_without_me(&self, sender: &mut SenderType, me: &SocketAddr, payload: Message) {
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
