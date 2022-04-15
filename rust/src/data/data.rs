use ckb_rocksdb::prelude::*;
use redis_rocksdb::RrError;

use crate::data::DataConfig;

pub struct Data {
    db: DB,
}

impl Data {
    pub fn init(config: &DataConfig) -> Result<Data, anyhow::Error> {
        Ok(Data::new(Data::make_db(config)?))
    }
    pub fn new(db: DB) -> Self {
        Data {
            db,
        }
    }

    pub fn make_db(config: &DataConfig) -> Result<DB, RrError> {
        let db = DB::open_default(&config.path)?;
        Ok(db)
    }
}