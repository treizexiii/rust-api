use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use tracing::info;
use tracing::log::debug;

use crate::model::ticket::TicketRepository;
use crate::web::AUTH_TOKEN;
use crate::ctx::Ctx;
use crate::{Error, Result};

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next) -> Result<Response> {
    debug!("{:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<TicketRepository>,
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token) {
        Ok((user_id, _exp, _sign)) => {
            info!("user_id:{:<12}",user_id);
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailTokenWrongFormat)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    request.extensions_mut().insert(result_ctx);

    Ok(next.run(request).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts.extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailNoContext)?
            .clone()
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}