pub mod config;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::network::simulation::tcp::TcpNetworkBundle;
use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
// use amethyst::Result as AmethystResult;
// use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

mod receive_message;
mod sender_message;
mod states;
mod tcp_socket;

pub use config::Config;
use receive_message::SpamReceiveBundle;
use states::GameState;
use tcp_socket::TCPSocket;

#[derive(Debug)]
pub struct Networking;

impl Networking {
    pub fn init(config: Config) {
        let socket_addr = config.addr.as_str();
        let socket = TCPSocket::new(socket_addr);
        let assets_dir = application_root_dir().unwrap().join("./");

        let buffer_size = config.buffer_size;
        let tcp_socket = Some(socket.socket);
        let tcp_bundle = TcpNetworkBundle::new(tcp_socket, buffer_size);

        let game_data = GameDataBuilder::default()
            .with_bundle(tcp_bundle)
            .unwrap()
            .with_bundle(SpamReceiveBundle)
            .unwrap();

        let duration = Duration::from_millis(2);
        let limit_strategy = FrameRateLimitStrategy::SleepAndYield(duration);
        let max_fps = 60;
        let mut game = Application::build(assets_dir, GameState)
            .unwrap()
            .with_frame_limit(limit_strategy, max_fps)
            .build(game_data)
            .unwrap();

        game.run();
    }
}
