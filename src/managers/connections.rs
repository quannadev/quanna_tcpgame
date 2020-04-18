use crate::managers::messages::MessageManager;
use amethyst::{ecs::Write, network::simulation::TransportResource, Result};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::str::from_utf8;

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    pub list_conn: Vec<SocketAddr>,
}
impl ConnectionManager {
    pub fn init() -> Self {
        ConnectionManager {
            list_conn: Vec::new(),
        }
    }
    pub fn on_connect<'a>(&mut self, addr: &SocketAddr, sender: &mut Write<'a, TransportResource>) {
        self.list_conn.push(*addr);
        let msg = format!("New connect {}\r\n", &addr);
        self.send_without_me(sender, addr, msg.as_str())
    }
    pub fn on_disconnect<'a>(
        &mut self,
        addr: &SocketAddr,
        sender: &mut Write<'a, TransportResource>,
    ) {
        let idx = self.list_conn.iter().position(|a| a == addr);
        if idx.is_some() {
            self.list_conn.remove(idx.unwrap());
            let msg = format!("Client {} disconnected \r\n", &addr);
            self.send_without_me(sender, addr, msg.as_str());
        }

        info!("Count socket {}", self.list_conn.len())
    }
    pub fn on_message<'a>(
        &mut self,
        addr: SocketAddr,
        payload: &[u8],
        sender: &mut Write<'a, TransportResource>,
    ) {
        let message_manager = MessageManager::init(sender);
        let message = message_manager.parser(payload);
        if message.is_some() {
            let msg_parsed = message.unwrap();
            sender.send(addr, msg_parsed.to_vec_u8().as_ref())
        } else {
            warn!(
                "message {} invalid",
                from_utf8(payload).unwrap().replace("\\", "")
            )
        }
    }
    pub fn send_all<'a>(&mut self, sender: &mut Write<'a, TransportResource>, payload: &str) {
        for socket in self.list_conn.iter() {
            Self::send_message(*socket, payload.as_bytes(), sender)
        }
    }
    pub fn send_without_me<'a>(
        &self,
        sender: &mut Write<'a, TransportResource>,
        me: &SocketAddr,
        payload: &str,
    ) {
        for socket in self.list_conn.iter() {
            if socket.eq(&me) {
                continue;
            }
            Self::send_message(*socket, payload.as_bytes(), sender)
        }
    }
    pub fn send_message<'a>(
        socket: SocketAddr,
        payload: &[u8],
        sender: &mut Write<'a, TransportResource>,
    ) {
        sender.send(socket, payload)
    }
}
