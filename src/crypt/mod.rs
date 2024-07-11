mod error;

pub use self::error::{ Error, Result };

use hmac::{ Hmac, Mac };
use sha2::Sha512;

pub struct EncryptContent {
    pub content: String,
    pub salt: String,
}

pub fn encrypt_into_b64u(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
    let EncryptContent { content, salt } = enc_content;

    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    let result = base64_url::encode(&result_bytes);

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::{Ok, Result};
    use rand::RngCore;

    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64];
        let fx_enc_content = EncryptContent {
            content: "test_encrypt_into_b64u_ok content".to_string(),
            salt: "test_encrypt_into_b64u_ok salt".to_string(),
        };
        
        let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        assert_eq!(res, fx_res);

        Ok(())
    }
}