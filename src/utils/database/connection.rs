use crate::utils::config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub(crate) fn get_connection_pool() -> Pool {
    let database_url = config::get_connection_string();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    return r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
}
