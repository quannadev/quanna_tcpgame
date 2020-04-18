use crate::network::Config;
use r2d2_redis::{
    r2d2::{Pool, PooledConnection},
    redis::{parse_redis_url, Commands},
    RedisConnectionManager,
};

type Result<T> = std::result::Result<T, ()>;
pub type RedisConn = PooledConnection<RedisConnectionManager>;
#[derive(Debug, Clone)]
pub struct RedisDb {
    pub client: Pool<RedisConnectionManager>,
}

impl RedisDb {
    pub fn init(addr: &str) -> Self {
        let redis_addr = parse_redis_url(&addr).unwrap();
        let manager = RedisConnectionManager::new(redis_addr).expect("Redis connect error");
        let pool = Pool::builder().max_size(10).build(manager).unwrap();
        info!("Redis connected at: {}", addr);
        RedisDb { client: pool }
    }
    fn init_config(&self) {
        //todo logic set config into cache
    }
    pub fn set(&self, msg: RedisMessage) -> Result<bool> {
        let mut conn = self.client.get().unwrap();
        match msg.tag {
            RedisTag::SET => {
                match conn.set::<String, String, usize>(msg.key.as_string().clone(), msg.value) {
                    Ok(_) => Ok(true),
                    _ => Err(()),
                }
            }
            RedisTag::HSET => match conn.hset::<String, String, String, usize>(
                msg.key.as_string().clone(),
                msg.field.unwrap(),
                msg.value,
            ) {
                Ok(_) => Ok(true),
                _ => Err(()),
            },
            RedisTag::HMSET => match conn.hset_multiple::<String, String, String, usize>(
                msg.key.as_string().clone(),
                &[(msg.field.unwrap(), msg.value)],
            ) {
                Ok(_) => Ok(true),
                _ => Err(()),
            },
            _ => {
                error!("Wrong tags");
                Err(())
            }
        }
    }

    pub fn set_incr(&self, key: RedisKeys, field: String, ms_expire: usize) {
        let mut conn = self.client.get().unwrap();
        let incr_key = format!("{}_{}", key.as_string(), field);
        let _set = conn.incr::<String, usize, usize>(incr_key.clone(), 1);
        if ms_expire > 0 {
            let _set_expire = conn.pexpire::<String, usize>(incr_key, ms_expire);
        }
    }
    pub fn get_incr(&self, key: RedisKeys, field: String) -> usize {
        let mut conn = self.client.get().unwrap();
        let incr_key = format!("{}_{}", key.as_string(), field);
        match conn.get::<String, usize>(incr_key) {
            Ok(value) => value,
            Err(_) => 0,
        }
    }

    pub fn get(&self, msg: RedisGetMessage) -> Result<String> {
        let mut conn = self.client.get().unwrap();
        match msg.tag {
            RedisTag::GET => match conn.get::<String, String>(msg.key.as_string()) {
                Ok(value) => Ok(value),
                _ => Err(()),
            },
            RedisTag::HGET => {
                match conn.hget::<String, String, String>(msg.key.as_string(), msg.field.unwrap()) {
                    Ok(value) => Ok(value),
                    _ => Err(()),
                }
            }
            RedisTag::HGETALL => match conn.hgetall::<String, String>(msg.key.as_string()) {
                Ok(value) => Ok(value),
                _ => Err(()),
            },
            _ => {
                error!("Wrong tags");
                Err(())
            }
        }
    }
    pub fn del(&self, msg: RedisMessage) -> Result<bool> {
        let mut conn = self.client.get().unwrap();
        match msg.tag {
            RedisTag::DEL => {
                let _del = conn.del::<String, usize>(msg.key.as_string());
                Ok(true)
            }
            RedisTag::HDEL => {
                let _hdel =
                    conn.hdel::<String, String, usize>(msg.key.as_string(), msg.field.unwrap());
                Ok(true)
            }
            _ => {
                error!("Wrong tags");
                Err(())
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum RedisTag {
    SET,
    HSET,
    SETBIT,
    HMSET,
    INCR,
    GET,
    HGET,
    HGETALL,
    GETBIT,
    DEL,
    HDEL,
}
#[derive(Clone, Debug)]
pub struct RedisMessage {
    pub tag: RedisTag,
    pub key: RedisKeys,
    pub field: Option<String>,
    pub value: String,
}
#[derive(Clone, Debug)]
pub struct RedisGetMessage {
    pub tag: RedisTag,
    pub key: RedisKeys,
    pub field: Option<String>,
}
#[derive(Clone, Debug)]
pub enum RedisKeys {
    BadClient,
    Blacklist,
    Users,
}
impl RedisKeys {
    pub fn as_string(&self) -> String {
        let prefix = Config::get_env("REDIS_PREFIX");
        match self {
            RedisKeys::BadClient => format!("{}_BadClient", &prefix).to_lowercase(),
            RedisKeys::Blacklist => format!("{}_Blacklist", &prefix).to_lowercase(),
            RedisKeys::Users => format!("{}_Users", &prefix).to_lowercase(),
            _ => String::new(),
        }
    }
}
