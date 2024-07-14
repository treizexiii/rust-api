use std::sync::Arc;
use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;
use crate::web::middlewares::auth::CtxExtractorError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    LoginFailUserNotFound,
    LoginFailUserHasNoPassword,
    LoginFailUserNotValidated { user_id: i64 },
    LoginFailPasswordNotMatching { user_id: i64 },

    AuthFailNoAuthToken,
    AuthFailTokenWrongFormat,
    AuthFailNoContext,

    TicketDeleteIdNotFound { id: u64 },

    CtxExt(CtxExtractorError),

    ModelError(),
    Model(model::Error),
    Crypt(crypt::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            LoginFail
            | LoginFailUserNotFound
            | LoginFailUserNotValidated { .. }
            | LoginFailPasswordNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }

            AuthFailNoAuthToken
            | AuthFailTokenWrongFormat
            | AuthFailNoContext => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            ModelError() | TicketDeleteIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl From<CtxExtractorError> for Error {
    fn from(value: CtxExtractorError) -> Self {
        Self::CtxExt(value)
    }
}

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}

impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter)
        -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

// --- CLIENT ERROR

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
