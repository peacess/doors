use std::{io, path};

use serde::{Deserialize, Serialize};

use crate::kits;

#[derive(Serialize, Deserialize, Default)]
pub struct DbConfig {
    pub path: String,
}

impl DbConfig {
    pub const DEFAULT_PATH: &'static str = "data_db";

    pub fn init(config: &mut DbConfig) -> Result<(), io::Error> {
        if config.path.is_empty() {
            config.path = DbConfig::DEFAULT_PATH.to_owned();
        }

        let mut p = path::PathBuf::from(&config.path);

        if !p.exists() {
            if !p.is_absolute() {
                match kits::Paths::run_path() {
                    Ok(t) => {
                        p = t.join(p);
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                        return Err(e);
                    }
                }
            }
            if !p.exists() {
                let _ = std::fs::create_dir_all(p)?;
            }
        }

        Ok(())
    }
}
