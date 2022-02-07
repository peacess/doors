use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_port")]
    port: i32,
    #[serde(default = "Config::default_ip")]
    ip: String,
}


impl Config {
    fn default_port() -> i32 { 8699 }
    fn default_ip() -> String { "::".to_string() }
    pub fn load(file: &str) -> Config {
        let config: Config = toml::from_str("").unwrap();
        config
    }
}