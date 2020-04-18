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
