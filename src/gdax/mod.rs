mod connection;

use error::*;

pub struct Gdax {
    pub connection: connection::Connection,
}

impl Gdax {
    pub fn connect(api_key: &str) -> Result<Gdax> {
        unimplemented!()
    }
}