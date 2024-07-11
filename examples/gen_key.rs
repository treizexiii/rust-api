use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut key);

    println!("HMAC Key: \n{:?}", key);

    let base64u = base64_url::encode(&key);
    println!("Base64 Key: \n{:?}", base64u);
    

    Ok(())
}