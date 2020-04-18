use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{result::Error, MysqlConnection};

pub type MysqlConn = PooledConnection<ConnectionManager<MysqlConnection>>;
pub type ResultDb<T> = std::result::Result<T, Error>;
#[derive(Clone)]
pub struct MysqlDb {
    pub conn: Pool<ConnectionManager<MysqlConnection>>,
}
impl MysqlDb {
    pub fn init(addr: &str) -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(addr);
        let pool = Pool::builder().build(manager).unwrap();
        info!("Mysql connected");
        MysqlDb { conn: pool }
    }
}
