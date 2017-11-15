use config::Config;
use discord::Discord;
use error::*;

pub struct Bot {
    config: Config,
    discord: Discord,
}

impl Bot {
    pub fn new(config: Config) -> Result<Self> {
        let discord = Discord::connect(&config.api_key)?;

        let bot = Bot {
            config,
            discord,
        };

        Ok(bot)
    }

    pub fn run(self) -> ! {
        loop {
            
        }
    }
}