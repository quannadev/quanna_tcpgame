use amethyst::ecs::Write;
use amethyst::network::simulation::TransportResource;
// use amethyst::Result;
// use std::collections::HashMap;
use std::net::SocketAddr;

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
        sender.send(addr, payload);
    }

    pub fn send_all<'a>(&mut self, sender: &mut Write<'a, TransportResource>, payload: &str) {
        let bytes = payload.as_bytes();
        let send_data = |s: &SocketAddr| Self::send_message(*s, bytes, sender);
        self.list_conn.iter().for_each(send_data);
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
