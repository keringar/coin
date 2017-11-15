pub mod connection;
pub mod model;

use error::*;

pub struct Discord {
    pub connection: connection::Connection,
}

impl Discord {
    pub fn connect(api_key: &str) -> Result<Self> {
        unimplemented!()
    }
}