#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate websocket;

mod bot;
mod gdax;
mod config;
mod error;

fn main() {
    // Load config file and write a default one if it fails
    let config = match config::Config::load_from_file() {
        Ok(config) => config,
        Err(e) => {
            use error_chain::ChainedError;
            
            // Print out original file read error
            eprintln!("{}", e.display_chain());

            // Print out any errors fron writing the file
            if let Err(err) = config::Config::write_default_to_file() {
                eprintln!("{}", &err.display_chain());
            }
            
            // Exit
            std::process::exit(-1);
        }
    };

    // Try and connect to GDAX and initialize bot
    let bot = match bot::Bot::new(config) {
        Ok(bot) => bot,
        Err(e) => {
            use error_chain::ChainedError;

            eprintln!("{}", e.display_chain());
            
            // Exit
            std::process::exit(-1);
        }
    };

    bot.run();
}