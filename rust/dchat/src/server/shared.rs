use std::sync::Arc;

use redis_rocksdb::RedisRocksdb;

use crate::data;
use crate::data::{BaseDao, Data, PartnerDao};
use crate::server::config::Config;

pub struct Shared {
    pub db: Arc<data::Data<RedisRocksdb>>,
    pub partner_dao: PartnerDao,
}

impl Shared {
    pub fn init(config: &Config) -> Result<Self, anyhow::Error> {
        let db = Data::init(&config.db)?;
        let db = Arc::new(db);

        Ok(Shared {
            db: db.clone(),
            partner_dao: PartnerDao::new(db.clone()),
        })
    }
}