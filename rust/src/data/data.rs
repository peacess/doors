use ckb_rocksdb::prelude::*;
use redis_rocksdb::{RedisList, RedisRocksdb, RrError};

use crate::data::DataConfig;

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

    pub fn make_db(config: &DataConfig) -> Result<ckb_rocksdb::TransactionDB, RrError> {
        let db = ckb_rocksdb::TransactionDB::open_default(&config.path)?;
        Ok(db)
    }
}