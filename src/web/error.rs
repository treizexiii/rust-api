use std::sync::Arc;
use crate::{model, web, pwd, token};
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

    RpcMethodUnknown(String),
    RpcMissingParams { rpc_method: String },
    RpcFailJsonParams { rpc_method: String },

    ModelError(),
    Model(model::Error),
    Pwd(pwd::Error),
    Token(token::Error),

    SerdeJson(String),
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

            Model(model::Error::EntityNotFound { entity, id }) => {
                (StatusCode::NOT_FOUND, ClientError::ENTITY_NOT_FOUND { entity, id: *id })
            }

            TicketDeleteIdNotFound { .. } => {
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

impl From<token::Error> for Error {
    fn from(value: token::Error) -> Self {
        Self::Token(value)
    }
}

impl From<pwd::Error> for Error {
    fn from(value: pwd::Error) -> Self {
        Self::Pwd(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value.to_string())
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
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
}
