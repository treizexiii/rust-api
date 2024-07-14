use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::log::debug;

use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{UserForLogin, UserRepository};
use crate::model::DbContext;
use crate::web;
use crate::web::AUTH_TOKEN;

use super::{Error, Result};

pub fn routes(db_context: DbContext) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .with_state(db_context)
}

async fn api_login(
    State(db_context): State<DbContext>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
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
        &EncryptContent {
            salt: user.pwd_salt.to_string(),
            content: pwd_clear.clone(),
        },
        &pwd,
    ).map_err(|_| Error::LoginFailPasswordNotMatching { user_id });

    web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

    let body = Json(json!({
        "result": {
            "succes": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
