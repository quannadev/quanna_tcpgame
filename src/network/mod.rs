use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::network::simulation::tcp::TcpNetworkBundle;
use amethyst::utils::application_root_dir;
// use amethyst::Result as AmethystResult;
use amethyst::prelude::*;

// use std::net::{SocketAddr, TcpStream};
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
        Self { config: config.clone() };
        Self::init(config)
    }

    fn init(config: Config) {
        let socket_addr = config.addr.as_str();
        let socket = TCPSocket::new(socket_addr);
        let assets_dir = application_root_dir().unwrap().join("./");

        let tcp_bundle = TcpNetworkBundle::new(
            Some(socket.socket),
            config.buffer_size,
        );

        let game_data = GameDataBuilder::default()
            .with_bundle(tcp_bundle)
            .unwrap()
            .with_bundle(SpamReceiveBundle)
            .unwrap();

        let frame_limit = FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2));
        let mut game = Application::build(assets_dir, GameState)
            .unwrap()
            .with_frame_limit(frame_limit, 60)
            .build(game_data)
            .unwrap();

        game.run();
    }
}
