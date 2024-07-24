use std::fmt::Formatter;
use derive_more::From;
use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Model(model::Error),

    FailToCreatePool { msg: String },
}

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
