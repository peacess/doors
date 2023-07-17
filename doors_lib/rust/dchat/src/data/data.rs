use redis_rocksdb::{RedisList, RedisRocksdb, RrError};

use crate::data::DataConfig;

pub type Id16 = u128;

pub struct Data<T> where T: RedisList {
    db: T,
}

impl Data<RedisRocksdb> {
    pub fn init(config: &DataConfig) -> Result<Self, anyhow::Error> {
        let db = Data::make_db(config)?;
        Ok(Data {
            db: RedisRocksdb::new(db),
        })
    }

    pub fn make_db(config: &DataConfig) -> Result<rocksdb::TransactionDB, RrError> {
        let db = rocksdb::TransactionDB::open_default(&config.path)?;
        Ok(db)
    }
}