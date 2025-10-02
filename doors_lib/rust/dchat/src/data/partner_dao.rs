use std::sync::Arc;

use redis_rocksdb::{RedisRocksdb, RrError};

use crate::{
    data,
    data::{Data, Id16, Partner, base_dao::BaseDao},
};

pub struct PartnerDao {
    db: Arc<data::Data<RedisRocksdb>>,
}

impl BaseDao<Partner> for PartnerDao {
    fn new(_db: Arc<Data<RedisRocksdb>>) -> Self {
        todo!()
    }

    fn get(&self, _id: &Id16) -> Result<Option<Partner>, RrError> {
        todo!()
    }
}

impl PartnerDao {
    pub fn get_self(&self) -> Result<Option<Partner>, RrError> {
        todo!()
    }
}
