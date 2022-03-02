use rocksdb::{DBWithThreadMode, MultiThreaded};

use crate::data::DataConfig;

pub struct Data {
    db: DBWithThreadMode<MultiThreaded>,
}

impl Data {
    pub fn init(config:&DataConfig) -> Result<Data, anyhow::Error> {
        Ok(Data::new(Data::make_db(config)?))
    }
    pub fn new(db: DBWithThreadMode<MultiThreaded>) -> Self {
        Data {
            db,
        }
    }

    pub fn make_db(config: &DataConfig) -> anyhow::Result<DBWithThreadMode<MultiThreaded>> {
        let db = rocksdb::DB::open_default(&config.path)?;
        Ok(db)
    }
}