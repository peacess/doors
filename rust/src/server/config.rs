use std::io;

use serde::{Deserialize, Serialize};

use crate::data::DataConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_port")]
    pub port: i32,
    #[serde(default = "Config::default_ip")]
    pub ip: String,
    #[serde(default = "Config::default_db")]
    pub db: DataConfig,
}


impl Config {
    fn default_port() -> i32 { 8699 }
    fn default_ip() -> String { "::".to_string() }
    fn default_db() -> DataConfig {
        DataConfig {
            path: DataConfig::DEFAULT_PATH.to_owned(),
        }
    }
    pub fn load(file: &str) -> Result<Config, io::Error> {
        let mut config: Config = toml::from_str("").unwrap();
        Config::init_default(&mut config)?;
        Ok(config)
    }
    pub fn init_default(config: &mut Config) -> Result<(), io::Error> {
        if config.ip.is_empty() {
            config.ip = "::".to_owned();
        }
        if config.port < 1 {
            config.port = 8699;
        }
        DataConfig::init(&mut config.db)?;
        Ok(())
    }
}