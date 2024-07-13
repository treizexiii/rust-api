use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use crate::ctx::Ctx;
use crate::model::store::{Db, new_db_pool};

mod unit_test;
mod error;
mod base;
mod store;
pub mod ticket;
pub mod task;
pub mod user;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct DbContext {
    db: Db,
}

impl DbContext {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(DbContext { db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
