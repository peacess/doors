use redis_rocksdb::RrError;

use crate::data::Partner;
use crate::idl::Int128;

impl Partner {
    pub fn get(id: &Int128) -> Result<Option<Partner>, RrError> {
        todo!()
    }
}