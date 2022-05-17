use redis_rocksdb::RrError;

use crate::data::{Id16, Partner};

impl Partner {
    pub fn get(id: &Id16) -> Result<Option<Partner>, RrError> {
        todo!()
    }
    pub fn get_self() -> Result<Option<Partner>, RrError> {
        todo!()
    }
}