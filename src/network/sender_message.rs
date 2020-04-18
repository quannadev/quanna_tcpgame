use amethyst::{ecs::Write, network::simulation::TransportResource};
use std::net::SocketAddr;

pub struct SenderManager<'a> {
    pub sender: Write<'a, TransportResource>,
}

impl<'a> SenderManager<'a> {
    pub fn init(sender: Write<'a, TransportResource>) -> Self {
        SenderManager { sender }
    }

    pub fn send(&mut self, addr: SocketAddr, payload: &[u8]) {
        self.sender.send(addr, payload);
    }
}
