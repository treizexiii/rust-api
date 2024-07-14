use std::fmt::Formatter;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    KeyFail,
    NotMatching,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
