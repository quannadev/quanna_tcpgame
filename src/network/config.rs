use amethyst::{start_logger, LoggerConfig, StdoutLog};
use log::LevelFilter;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: String,
    pub ws_addr: String,
    pub buffer_size: usize,
    pub redis_uri: String,
    pub redis_prefix: String,
    pub mysql_uri: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv::dotenv().ok();
        let addr = "127.0.0.1:4567".to_string();
        let ws_addr = "127.0.0.1:4568".to_string();
        let redis_uri = Self::get_env("REDIS_URI");
        let redis_prefix = Self::get_env("REDIS_PREFIX");
        let mysql_uri = Self::get_env("DATABASE_URL");
        Self::internal_init(addr, redis_uri, redis_prefix, mysql_uri, ws_addr)
    }
}

impl Config {
    pub fn new(addr: String, ws_addr: String) -> Self {
        dotenv::dotenv().ok();
        let redis_uri = Self::get_env("REDIS_URI");
        let redis_prefix = Self::get_env("REDIS_PREFIX");
        let mysql_uri = Self::get_env("DATABASE_URL");
        Self::internal_init(addr, redis_uri, redis_prefix, mysql_uri, ws_addr)
    }

    pub fn logger_config() -> LoggerConfig {
        LoggerConfig {
            allow_env_override: true,
            level_filter: LevelFilter::Info,
            log_file: None,
            log_gfx_backend_level: Some(LevelFilter::Warn),
            log_gfx_rendy_level: Some(LevelFilter::Warn),
            module_levels: vec![],
            stdout: StdoutLog::Colored,
        }
    }

    pub fn get_env(key: &str) -> String {
        let panic_msg = format!("Missing env var {}", key);
        let dotenv_get = |_| dotenv::var(&key).expect(&panic_msg);
        env::var(&key).unwrap_or_else(dotenv_get)
    }

    fn internal_init(
        addr: String,
        redis_uri: String,
        redis_prefix: String,
        mysql_uri: String,
        ws_addr: String,
    ) -> Self {
        let cfg = Self::logger_config();
        start_logger(cfg);

        Self {
            addr,
            buffer_size: 2048,
            redis_uri,
            redis_prefix,
            mysql_uri,
            ws_addr,
        }
    }
}
