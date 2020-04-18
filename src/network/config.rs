use amethyst::{start_logger, LoggerConfig, StdoutLog};
use log::LevelFilter;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: String,
    pub buffer_size: usize,
    pub redis_uri: String,
    pub redis_prefix: String,
    pub mysql_uri: String,
}
impl Default for Config {
    fn default() -> Self {
        dotenv::dotenv().ok();
        let addr = "127.0.0.1:4567".to_string();
        let redis_uri = Self::get_env("REDIS_URI");
        let redis_prefix = Self::get_env("REDIS_PREFIX");
        let mysql_uri = Self::get_env("DATABASE_URL");
        Self::internal_init(addr, redis_uri, redis_prefix, mysql_uri)
    }
}
impl Config {
    pub fn new(addr: String) -> Self {
        dotenv::dotenv().ok();
        let redis_uri = Self::get_env("REDIS_URI");
        let redis_prefix = Self::get_env("REDIS_PREFIX");
        let mysql_uri = Self::get_env("DATABASE_URL");
        Self::internal_init(addr, redis_uri, redis_prefix, mysql_uri)
    }
    pub fn logger_config() -> LoggerConfig {
        LoggerConfig {
            stdout: StdoutLog::Colored,
            level_filter: LevelFilter::Info,
            log_file: None,
            allow_env_override: true,
            log_gfx_backend_level: Some(LevelFilter::Warn),
            log_gfx_rendy_level: Some(LevelFilter::Warn),
            module_levels: vec![],
        }
    }
    pub fn get_env(key: &str) -> String {
        match env::var(&key) {
            Ok(value) => value,
            Err(_) => match dotenv::var(&key) {
                Ok(value) => value,
                Err(_) => panic!("Missing key in env"),
            },
        }
    }
    fn internal_init(
        addr: String,
        redis_uri: String,
        redis_prefix: String,
        mysql_uri: String,
    ) -> Self {
        start_logger(Self::logger_config());
        Self {
            addr,
            buffer_size: 2048,
            redis_uri,
            redis_prefix,
            mysql_uri,
        }
    }
}
