use crate::database::enums::{RedisGetMessage, RedisKeys, RedisMessage, RedisTag};
use crate::database::{MysqlConn, RedisDb, ResultDb};
use crate::models::CRUD;
use crate::schema::users;
use crate::schema::users::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{delete, insert_into, update};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(Queryable, Eq, PartialEq, Debug, Deserialize, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn parse_struct(txt: String) -> Option<Self> {
        match serde_json::from_str::<Self>(txt.as_str()) {
            Ok(user) => Some(user),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn login(form: &NewUser, conn: &MysqlConn, redis: RedisDb) -> Option<Self> {
        match Self::find_by_name(form.username.clone(), conn, redis) {
            Ok(user) => {
                if user.verify_password(form.password.as_str()) {
                    return Some(user);
                }
                None
            }
            _ => None,
        }
    }
    fn verify_password(&self, passwd: &str) -> bool {
        self.password.eq(passwd)
    }
}

#[derive(Insertable, AsChangeset, Eq, PartialEq, Debug, Deserialize, Clone, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn new(user_name: String, passwd: String) -> Self {
        NewUser {
            username: user_name,
            password: passwd,
        }
    }

    pub fn parser_from_str(txt: &str) -> Option<Self> {
        match serde_json::from_str::<Self>(txt.to_lowercase().as_str()) {
            Ok(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn validate(&self) -> bool {
        self.username.len() > 3 && self.password.len() > 5
    }
}

impl CRUD<NewUser> for User {
    fn find_all(limit: i64, conn: &MysqlConn, _redis: RedisDb) -> ResultDb<Vec<Self>> {
        users.limit(limit).load::<Self>(conn)
    }

    fn find_by_id(uid: i32, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self> {
        use crate::schema::users::dsl::*;
        let cache = redis.get(RedisGetMessage {
            tag: RedisTag::HGET,
            key: RedisKeys::Users,
            field: Some(uid.clone().to_string()),
        });
        if cache.is_ok() {
            let txt = cache.unwrap();
            let user = User::parse_struct(txt);
            if user.is_some() {
                return Ok(user.unwrap());
            }
        }
        match users.find(uid).first::<Self>(conn) {
            Ok(user) => {
                let user_data = user.clone();
                let _ = redis.set(RedisMessage {
                    tag: RedisTag::HMSET,
                    key: RedisKeys::Users,
                    field: Some(user_data.id.to_string()),
                    value: user_data.to_string(),
                });
                Ok(user)
            }
            _ => Err(DieselError::NotFound),
        }
    }

    fn find_by_name(name: String, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self> {
        use crate::schema::users::dsl::*;
        let cache = redis.get(RedisGetMessage {
            tag: RedisTag::HGET,
            key: RedisKeys::Users,
            field: Some(name.clone()),
        });
        if cache.is_ok() {
            let txt = cache.unwrap();
            let user = User::parse_struct(txt);
            if user.is_some() {
                return Ok(user.unwrap());
            }
        }
        match users.filter(username.eq(name)).first::<Self>(conn) {
            Ok(user) => {
                let user_data = user.clone();
                let _ = redis.set(RedisMessage {
                    tag: RedisTag::HMSET,
                    key: RedisKeys::Users,
                    field: Some(user_data.username.clone()),
                    value: user_data.to_string(),
                });
                Ok(user)
            }
            _ => Err(DieselError::NotFound),
        }
    }

    fn insert(value: &NewUser, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self> {
        let check = Self::find_by_name(value.username.clone(), conn, redis.clone());
        if check.is_ok() {
            return Err(DieselError::NotFound);
        }
        let user = insert_into(users).values(value.clone()).execute(conn);
        match user {
            Ok(user) => {
                let _ = redis.del(RedisMessage {
                    tag: RedisTag::HDEL,
                    key: RedisKeys::Users,
                    field: Some(value.username.clone()),
                    value: "".to_string(),
                });
                return Self::find_by_name(value.username.clone(), conn, redis);
            }
            _ => Err(DieselError::NotFound),
        }
    }

    fn update(value: &NewUser, uid: i32, conn: &MysqlConn, redis: RedisDb) -> ResultDb<Self> {
        use crate::schema::users::dsl::*;
        match update(users.find(uid)).set(value).execute(conn) {
            Ok(user) => {
                let _ = redis.del(RedisMessage {
                    tag: RedisTag::HDEL,
                    key: RedisKeys::Users,
                    field: Some(uid.to_string()),
                    value: "".to_string(),
                });
                return Self::find_by_id(uid, conn, redis);
            }
            _ => Err(DieselError::NotFound),
        }
    }

    fn delete(uid: i32, conn: &MysqlConn, redis: RedisDb) -> ResultDb<usize> {
        use crate::schema::users::dsl::*;
        let _ = redis.del(RedisMessage {
            tag: RedisTag::HDEL,
            key: RedisKeys::Users,
            field: Some(uid.to_string()),
            value: "".to_string(),
        });
        delete(users.find(uid)).execute(conn)
    }
}
