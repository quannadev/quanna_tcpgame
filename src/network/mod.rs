use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, network::simulation::tcp::TcpNetworkBundle,
    prelude::*, utils::application_root_dir, Result,
};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub mod config;
mod receive_message;
mod states;
mod tcp_socket;

use crate::database::{MysqlDb, RedisDb};
pub use config::Config;
use receive_message::SpamReceiveBundle;
use states::GameState;
use tcp_socket::TCPSocket;

pub struct Networking {
    pub config: Config,
    pub redis: RedisDb,
    pub mysql: MysqlDb,
}
impl Networking {
    pub fn new(config: Config) {
        let redis = RedisDb::init(config.redis_uri.as_str());
        let mysql = MysqlDb::init(config.mysql_uri.as_str());
        Self {
            config: config.clone(),
            redis: redis.clone(),
            mysql: mysql.clone(),
        };
        Self::init(config, redis, mysql)
    }
    fn init(config: Config, redis: RedisDb, mysql: MysqlDb) {
        let socket = TCPSocket::new(config.addr.as_str());
        let assets_dir = application_root_dir().unwrap().join("./");
        let game_data = GameDataBuilder::default()
            .with_bundle(TcpNetworkBundle::new(
                Some(socket.socket),
                config.buffer_size,
            ))
            .unwrap()
            .with_bundle(SpamReceiveBundle { redis, mysql })
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
