use crate::crypt::token::{validate_web_token, Token};
use crate::ctx::Ctx;
use crate::model::user::{UserRepository, UserForAuth};
use crate::model::DbContext;
use crate::web::{set_token_cookie, AUTH_TOKEN};
use crate::web::{Error, Result};

use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use crate::web::Error::CtxExt;

#[allow(dead_code)]
pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    db_context: State<DbContext>,
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(db_context, &cookies).await;

    if ctx_ext_result.is_err() &&
        !matches!(ctx_ext_result, Err(CtxExtractorError::TokenNotInCookie))
    {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    request.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(request).await)
}

async fn _ctx_resolve(
    State(db_context): State<DbContext>,
    cookies: &Cookies)
    -> CtxExtractorResult {
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtractorError::TokenNotInCookie)?;

    let token = token.parse::<Token>().map_err(|_| CtxExtractorError::TokenWrongFormat)?;

    let user: UserForAuth = UserRepository::
    first_by_username(&Ctx::root_ctx(), &db_context, &token.identifier)
        .await
        .map_err(|ex| CtxExtractorError::DbContextAccessError(ex.to_string()))?
        .ok_or(CtxExtractorError::UserNotFound)?;

    validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| CtxExtractorError::FailValidateToken)?;

    set_token_cookie(cookies, &user.username, &user.token_salt.to_string())
        .map_err(|_| CtxExtractorError::CannotSetTokenCookie);

    Ctx::new(user.id).map_err(|ex| CtxExtractorError::CtxCreateFail(ex.to_string()))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtractorResult>()
            .ok_or(Error::CtxExt(CtxExtractorError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtractorResult = core::result::Result<Ctx, CtxExtractorError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtractorError {
    TokenNotInCookie,
    TokenWrongFormat,
    UserNotFound,
    DbContextAccessError(String),
    FailValidateToken,
    CannotSetTokenCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}