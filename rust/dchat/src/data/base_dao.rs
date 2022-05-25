use std::sync::Arc;

use redis_rocksdb::{RedisRocksdb, RrError};

use crate::data;
use crate::data::Id16;

pub trait BaseDao<T> {
    fn new(db: Arc<data::Data<RedisRocksdb>>) -> Self;
    fn get(&self, id: &Id16) -> Result<Option<T>, RrError>;
}