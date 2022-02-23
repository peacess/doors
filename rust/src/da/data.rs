use rocksdb::{DBWithThreadMode, MultiThreaded};

use crate::da::DbConfig;

pub struct Data {
    db: DBWithThreadMode<MultiThreaded>,
}

impl Data {
    pub fn new(db: DBWithThreadMode<MultiThreaded>) -> Self {
        Data {
            db,
        }
    }

    pub fn make_db(config: &DbConfig) -> anyhow::Result<DBWithThreadMode<MultiThreaded>> {
        let db = rocksdb::DB::open_default("")?;
        Ok(db)
    }
}