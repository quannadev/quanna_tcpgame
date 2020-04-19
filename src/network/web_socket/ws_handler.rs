use crate::network::WebsocketServer;
use ws::{CloseCode, Handler, Handshake, Message, Result};

impl Handler for WebsocketServer {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(addr) = shake.remote_addr()? {
            debug!("Connection with {} now open", addr);
        }
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.sender.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for aaa ({:?}) {}", code, reason);
        // self.sender.shutdown().unwrap();
    }
}
