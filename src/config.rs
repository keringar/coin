use std::fs::File;
use std::io::prelude::*;

use error::*;
use discord::model::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub owners: Vec<UserId>,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config> {
        let mut config_file = File::open(path)
            .chain_err(|| ErrorKind::ConfigLoad(path.to_string()))?;

        let mut data = String::new();
        config_file.read_to_string(&mut data)
            .chain_err(|| format!("'{}' is not valid", path.to_string()))?;

        let config: Config = serde_json::from_str(&data)
            .chain_err(|| ErrorKind::ConfigParse(path.to_string()))?;

        Ok(config)
    }

    pub fn write_default_to_file(path: &str) -> Result<Config> {
        let config = Config::default();
        
        let json_str = serde_json::to_string_pretty(&config)
            .chain_err(|| ErrorKind::ConfigParse(path.to_string()))?;

        let mut config_file = File::create(path)
            .chain_err(|| ErrorKind::ConfigLoad(path.to_string()))?;

        config_file.write_all(&json_str.as_bytes())
            .chain_err(|| format!("Default config could not be written as utf-8"))?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: "NOT_SET".to_string(),
            owners: vec!(),
        }
    }   
}