use crate::model::store;
use crate::pwd;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Formatter;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    #[from]
    Pwd(pwd::Error),
    #[from]
    Store(store::Error),

    EntityNotFound { entity: &'static str, id: i64 },
    TicketDeleteIdNotFound { id: u64 },

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error)
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
