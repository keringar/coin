use error::*;
use std::fs::File;
use std::io::prelude::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn load_from_file() -> Result<Config> {
        let mut config_file = File::open("./config.json")
            .chain_err(|| "Unable to open config file")?;

        let mut data = String::new();
        config_file.read_to_string(&mut data)
            .chain_err(|| "Unable to read config file")?;

        let config: Config = serde_json::from_str(&data)
            .chain_err(|| "Unable to parse config file")?;

        Ok(config)
    }

    pub fn write_default_to_file() -> Result<Config> {
        let config = Config::default();
        
        let json_str = serde_json::to_string_pretty(&config)
            .chain_err(|| "Could not format default config file")?;

        let mut config_file = File::create("./example.json")
            .chain_err(|| "Could not create default config file")?;

        config_file.write_all(&json_str.as_bytes())
            .chain_err(|| "Could not write default config file")?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: "NOT_SET".to_string(),
        }
    }   
}