use crate::config::Config;
use crate::data;
use crate::data::Data;

pub struct Shared {
    pub db: data::Data,
}

impl Shared {
    pub fn init(config:&Config) -> Result<Self, anyhow::Error> {
        let db = Data::init(&config.db)?;

        Ok(Shared{
            db,
        })
    }
}