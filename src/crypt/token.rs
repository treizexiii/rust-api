use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent, Error, Result};
use crate::utils::{b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc};

// string format: `id_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
    pub identifier: String,
    pub expiration: String,
    pub signature: String,
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();

        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }

        let (ident, exp, sign) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            identifier: b64u_decode(ident).map_err(|_| Error::TokenCannotDecodeIdentifier)?,
            expiration: b64u_decode(exp).map_err(|_| Error::TokenCannotDecodeExpiration)?,
            signature: sign.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{}.{}.{}",
               b64u_encode(&self.identifier),
               b64u_encode(&self.expiration),
               self.signature)
    }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = &config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

// PRIVATE REGION
fn _generate_token(
    identifier: &str,
    duration_sec: f64,
    salt: &str,
    key: &[u8])
    -> Result<Token> {
    let ident = identifier.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        identifier: ident,
        expiration: exp,
        signature: sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    let new_sign_b64u =
        _token_sign_into_b64u(&origin_token.identifier, &origin_token.expiration, salt, key)?;

    if new_sign_b64u != origin_token.signature {
        return Err(Error::TokenSignatureNotMatching);
    }

    let origin_exp = parse_utc(&origin_token.expiration).map_err(|_| Error::TokenExpirationNotIdo)?;
    let now = now_utc();
    if origin_exp < now {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

fn _token_sign_into_b64u(
    identifier: &str,
    expiration: &str,
    salt: &str,
    key: &[u8])
    -> Result<String> {
    let content = format!("{}.{}", b64u_encode(identifier), b64u_encode(expiration));
    let signature = encrypt_into_b64u(key, &EncryptContent {
        content,
        salt: salt.to_string()
    })?;

    Ok(signature)
}

// region: -- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]

    use std::thread;
    use std::time::Duration;
    use super::*;
    use anyhow::Result;

    #[test]
    pub fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZngtaW5kZW50aWZpZXItMDE.MjAyNC0wMS0xNFQxMTo0NTowMFo.some-signature-b64u-encoded";
        let fx_token = Token {
            identifier: "fx-indentifier-01".to_string(),
            expiration: "2024-01-14T11:45:00Z".to_string(),
            signature: "some-signature-b64u-encoded".to_string(),
        };

        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }

    #[test]
    pub fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = "ZngtaW5kZW50aWZpZXItMDE.MjAyNC0wMS0xNFQxMTo0NTowMFo.some-signature-b64u-encoded";

        let fx_token = Token {
            identifier: "fx-indentifier-01".to_string(),
            expiration: "2024-01-14T11:45:00Z".to_string(),
            signature: "some-signature-b64u-encoded".to_string(),
        };

        let token = Token::from_str(fx_token_str)?;

        assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));

        Ok(())
    }

    #[test]
    pub fn validate_zeb_token_ok() -> Result<()> {
        let fx_user = "user-01";
        let fx_salt = "salt-user-01";
        let fx_exp = 0.02;

        let token_key = &config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user,fx_exp,fx_salt,token_key)?;

        thread::sleep(Duration::from_millis(10));
        let res = validate_web_token(&fx_token, fx_salt);

        res?;

        Ok(())
    }

    #[test]
    pub fn validate_zeb_token_expired() -> Result<()> {
        let fx_user = "user-01";
        let fx_salt = "salt-user-01";
        let fx_exp = 0.02;

        let token_key = &config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user,fx_exp,fx_salt,token_key)?;

        thread::sleep(Duration::from_millis(30));
        let res = validate_web_token(&fx_token, fx_salt);

        assert!(
            matches!(res, Err(Error::TokenExpired)),
            "Should have matched `Err(Error::TokenExpired)` but have `{res:?}`"
        );

        Ok(())
    }
}
// endregion: -- Tests
