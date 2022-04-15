use crate::data;
use crate::data::Data;
use crate::server::config::Config;

pub struct Shared {
    pub db: data::Data,
}

impl Shared {
    pub fn init(config: &Config) -> Result<Self, anyhow::Error> {
        let db = Data::init(&config.db)?;

        Ok(Shared {
            db,
        })
    }
}