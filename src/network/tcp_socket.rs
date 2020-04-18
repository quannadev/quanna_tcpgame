use std::net::TcpListener;

#[derive(Debug)]
pub struct TCPSocket {
    pub socket: TcpListener,
}

impl TCPSocket {
    pub fn new(addr: &str) -> Self {
        let socket = TcpListener::bind(addr).expect("Bind tcp socket error");
        socket.set_nonblocking(true).unwrap();
        Self { socket }
    }
}
