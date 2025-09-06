pub use idl::base_generated::idl as base_generated;

pub mod data;
pub mod idl;
pub mod kits;
pub mod server;

use redis_rocksdb::rocksdb;
