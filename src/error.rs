use std::fmt::Formatter;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use strum_macros::AsRefStr;
use tracing::debug;
use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    ConfigMissingEnv(&'static str),

    LoginFail,

    AuthFailNoAuthToken,
    AuthFailTokenWrongFormat,
    AuthFailNoContext,

    TicketDeleteIdNotFound { id: u64 },

    FailToCreatePool { msg: String },

    ModelError()
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::ModelError()
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Error::LoginFail
            => { (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL) }

            Error::AuthFailNoAuthToken |
            Error::AuthFailTokenWrongFormat |
            Error::AuthFailNoContext
            => { (StatusCode::FORBIDDEN, ClientError::NO_AUTH) }

            Error::TicketDeleteIdNotFound { .. }
            => { (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS) }

            _
            => { (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR) }
        }
    }
}

// --- CLIENT ERROR

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
