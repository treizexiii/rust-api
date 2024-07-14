use std::fmt::Formatter;
use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    ConfigInvalidFormat(&'static str),

    Model(model::Error),

    FailToCreatePool { msg: String },
}

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
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