use crate::utils::config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub(crate) fn connect() -> ConnectionResult<PgConnection> {
    return PgConnection::establish(&config::get_connection_string());
}
