use std::fmt::Formatter;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    Store(store::Error),
    EntityNotFound { entity: &'static str, id: i64},
    TicketDeleteIdNotFound { id: u64 },
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}