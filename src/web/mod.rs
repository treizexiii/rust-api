use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;
use crate::token::generate_web_token;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub(crate) mod error;
pub mod routes_login;
pub mod routes_tickets;
pub mod routes_static;
pub mod middlewares;
pub mod rpc;

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");
    cookies.remove(cookie);

    Ok(())
}
