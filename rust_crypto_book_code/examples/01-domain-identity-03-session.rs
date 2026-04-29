use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};
use rand::RngCore;

fn encrypt_session(
    plaintext: &[u8],
    key_bytes: [u8; 32],
) -> Result<(Vec<u8>, [u8; 12]), Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| chacha20poly1305::Error)
        .unwrap();

    Ok((ciphertext, nonce_bytes))
}

fn decrypt_session(
    ciphertext: &[u8],
    nonce_bytes: [u8; 12],
    key_bytes: [u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| chacha20poly1305::Error)
        .unwrap();

    Ok(plaintext)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = [7u8; 32];
    let session_data = br#"{"sub":"alice","csrf":"abc123","mfa":"pending"}"#;

    let (ciphertext, nonce) = encrypt_session(session_data, key)?;
    println!("Ciphertext length: {}", ciphertext.len());

    let plaintext = decrypt_session(&ciphertext, nonce, key)?;
    println!("Decrypted session: {}", String::from_utf8(plaintext)?);

    Ok(())
}
