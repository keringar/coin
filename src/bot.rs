use config::Config;
use gdax::Gdax;
use error::*;

pub struct Bot {
    config: Config,
    gdax: Gdax,
}

impl Bot {
    pub fn new(config: Config) -> Result<Bot> {
        let gdax = Gdax::connect(&config.api_key)?;

        let bot = Bot {
            config,
            gdax,
        };

        Ok(bot)
    }

    pub fn run(self) -> ! {
        loop {
            
        }
    }
}