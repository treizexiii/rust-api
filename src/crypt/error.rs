use std::fmt::Formatter;
use serde::Serialize;
use crate::{crypt, model};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    KeyFailHmac,
    PwdInvalid,
    TokenInvalidFormat,
    TokenCannotDecodeIdentifier,
    TokenCannotDecodeExpiration,
    TokenSignatureNotMatching,
    TokenExpirationNotIdo,
    TokenExpired
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}