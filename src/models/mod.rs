mod messages_noti;
mod peer;
mod peer_status;
mod server_message;
mod user;
use crate::database::{MysqlConn, RedisDb, ResultDb};
pub use messages_noti::{MessageErrors, MessageSuccess};
pub use peer::Peer;
pub use peer_status::PeerStatus;
pub use server_message::ServerMessage;
pub use user::{NewUser, User};

pub trait CRUD<T> {
    fn find_all(limit: i64, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Vec<Self>>
    where
        Self: Sized;
    fn find_by_id(id: i32, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self>
    where
        Self: Sized;
    fn find_by_name(name: String, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self>
    where
        Self: Sized;
    fn insert(value: &T, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self>
    where
        Self: Sized;
    fn update(value: &T, id: i32, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self>
    where
        Self: Sized;
    fn delete(id: i32, conn: &MysqlConn, redis: RedisDb) -> ResultDb<usize>
    where
        Self: Sized;
}
