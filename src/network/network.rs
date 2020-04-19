use crate::database::{MysqlDb, RedisDb};
use crate::managers::connections::ConnectionManager;
use crate::network::{
    receive_message::TCPReceiveConnection, states::GameState, tcp_socket::TCPSocket, Config,
    WebsocketServer,
};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::network::simulation::tcp::TcpNetworkBundle;
use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Networking {
    pub config: Config,
    pub redis: RedisDb,
    pub mysql: MysqlDb,
    pub connect_manager: ConnectionManager,
}

impl Networking {
    pub fn new(config: Config) {
        let redis = RedisDb::init(config.redis_uri.as_str());
        let mysql = MysqlDb::init(config.mysql_uri.as_str());
        let server = Self {
            config: config.clone(),
            redis: redis.clone(),
            mysql: mysql.clone(),
            connect_manager: ConnectionManager::init(redis.clone(), mysql.clone()),
        };
        let tcp_start = thread::spawn(move || server.tcp_server());
        let _ = tcp_start.join();
    }
    fn tcp_server(&self) {
        let socket = TCPSocket::new(self.config.addr.as_str());
        let assets_dir = application_root_dir().unwrap().join("./");
        let buffer_size = self.config.buffer_size;
        let tcp_socket = Some(socket.socket);
        let tcp_bundle = TcpNetworkBundle::new(tcp_socket, buffer_size);
        let game_data = GameDataBuilder::default()
            .with_bundle(tcp_bundle)
            .unwrap()
            .with_bundle(TCPReceiveConnection {
                connection: self.connect_manager.clone(),
            })
            .unwrap();
        let mut game_server = Application::build(assets_dir, GameState)
            .unwrap()
            .build(game_data)
            .unwrap();

        game_server.run();
    }
    // fn start_ws_server(&self) -> JoinHandle<()> {
    //     thread::spawn(move || WebsocketServer::start(ws_addr.as_str()))
    // }
}
