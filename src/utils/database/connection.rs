use crate::utils::config;
use postgres::{Client, Error, NoTls};

pub(crate) fn connect() -> Result<(), Error> {
    let client = Client::connect(&config::get_connection_string(), NoTls)?;
    Ok(())
}
