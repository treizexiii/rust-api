use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::log::debug;

use crate::pwd::{self, ContentToHash};
use crate::ctx::Ctx;
use crate::model::user::{UserForLogin, UserRepository};
use crate::model::DbContext;
use crate::web;
use crate::web::AUTH_TOKEN;

use super::{Error, Result};

pub fn routes(db_context: DbContext) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/logout", post(api_logout))
        .with_state(db_context)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(
    State(db_context): State<DbContext>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>) -> Result<Json<Value>>
{
    debug!("{:<12} - api_login", "HANDLER");

    let LoginPayload {
        username,
        password: pwd_clear,
    } = payload;

    let root_ctx = Ctx::root_ctx();

    let user: UserForLogin = UserRepository::first_by_username(&root_ctx, &db_context, &username)
        .await?
        .ok_or(Error::LoginFailUserNotFound)?;
    let user_id = user.id;
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPassword)
    };

    pwd::validate_pwd(
        &ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        &pwd,
    ).map_err(|_| Error::LoginFailPasswordNotMatching { user_id });

    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    let body = Json(json!({
        "result": {
            "succes": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    logout: bool,
}

async fn api_logout(cookies: Cookies, Json(payload): Json<LogoutPayload>) -> Result<Json<Value>> {
    debug!("{:<12} - api_logout", "HANDLER");
    let should_logoff = payload.logout;

    if should_logoff {
        web::remove_token_cookie(&cookies);
    }

    let body = Json(json!({
        "result": {
            "succes": true
        }
    }));

    Ok(body)
}
