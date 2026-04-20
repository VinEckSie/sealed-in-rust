use ::hmac::{Hmac, Mac};
use ::sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

type HmacSha256 = Hmac<Sha256>;

fn sign_token(payload: &str, secret: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(payload.as_bytes());

    let tag = mac.finalize().into_bytes();

    let payload_b64 = URL_SAFE_NO_PAD.encode(payload.as_bytes());
    let tag_b64 = URL_SAFE_NO_PAD.encode(tag);

    Ok(format!("{payload_b64}.{tag_b64}"))
}

fn verify_token(token: &str, secret: &[u8]) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let Some((payload_b64, tag_b64)) = token.split_once('.') else {
        return Ok(None);
    };

    let payload = match URL_SAFE_NO_PAD.decode(payload_b64) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };

    let tag = match URL_SAFE_NO_PAD.decode(tag_b64) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };

    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(&payload);

    if mac.verify_slice(&tag).is_ok() {
        Ok(Some(String::from_utf8(payload)?))
    } else {
        Ok(None)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = b"server-secret-key";
    let payload = r#"{"sub":"alice","role":"user","exp":1735689600}"#;

    let token = sign_token(payload, secret)?;
    println!("Token:\n{token}\n");

    let verified = verify_token(&token, secret)?;
    println!("Verified payload: {verified:?}");

    let tampered_token = format!("x{}", &token[1..]);
    let verified_tampered = verify_token(&tampered_token, secret)?;
    println!("Tampered token accepted: {verified_tampered:?}");

    Ok(())
}
