use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, network::simulation::tcp::TcpNetworkBundle,
    prelude::*, utils::application_root_dir, Result,
};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub mod config;
mod receive_message;
mod sender_message;
mod states;
mod tcp_socket;

pub use config::Config;
use receive_message::SpamReceiveBundle;
use states::GameState;
use tcp_socket::TCPSocket;

#[derive(Debug)]
pub struct Networking {
    pub config: Config,
}
impl Networking {
    pub fn new(config: Config) {
        Self {
            config: config.clone(),
        };
        Self::init(config)
    }
    fn init(config: Config) {
        let socket = TCPSocket::new(config.addr.as_str());
        let assets_dir = application_root_dir().unwrap().join("./");
        let game_data = GameDataBuilder::default()
            .with_bundle(TcpNetworkBundle::new(
                Some(socket.socket),
                config.buffer_size,
            ))
            .unwrap()
            .with_bundle(SpamReceiveBundle)
            .unwrap();
        let mut game = Application::build(assets_dir, GameState)
            .unwrap()
            .with_frame_limit(
                FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
                60,
            )
            .build(game_data)
            .unwrap();
        game.run();
    }
}
