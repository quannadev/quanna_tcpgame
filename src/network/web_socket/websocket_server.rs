use crate::network::Config;
use std::thread;
use ws::{listen, CloseCode, Handler, Message, Result, Sender};

pub struct WebsocketServer {
    pub sender: Sender,
}
impl WebsocketServer {
    pub fn init(sender: Sender) -> Self {
        Self { sender }
    }
    pub fn start(addr: &str) {
        listen(addr, |sender| {
            // info!("Websocket started at: {}", &config.ws_addr);
            WebsocketServer::init(sender)
        })
        .unwrap();
    }
}
