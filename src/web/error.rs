use std::sync::Arc;
use crate::{model, web, pwd, token};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use tracing::debug;
use crate::web::middlewares::auth::CtxExtractorError;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
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

    #[from]
    CtxExt(CtxExtractorError),

    RpcMethodUnknown(String),
    RpcMissingParams { rpc_method: String },
    RpcFailJsonParams { rpc_method: String },

    ModelError(),
    #[from]
    Model(model::Error),
    #[from]
    Pwd(pwd::Error),
    #[from]
    Token(token::Error),

    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),}

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
