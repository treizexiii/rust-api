mod error;

pub use error::{Error, Result};

use crate::config::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    let max_connections = if cfg!(test) { 1 } else { 5 };
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
