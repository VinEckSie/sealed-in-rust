use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

fn main() {
    let key = *b"an example very very secret key!";
    let nonce = *b"unique nonce";
    let text = b"secret meeting at midnight";

    let mut ciphertext = text.clone();

    let mut encryptor = ChaCha20::new(&key.into(), &nonce.into());
    encryptor.apply_keystream(&mut ciphertext);

    println!("Ciphertext (hex): {}", hex::encode(&ciphertext));

    let mut decrypted = ciphertext.clone();
    let mut decryptor = ChaCha20::new(&key.into(), &nonce.into());
    decryptor.apply_keystream(&mut decrypted);

    println!("Decrypted text: {}", String::from_utf8_lossy(&decrypted));

    assert_eq!(text, &decrypted);
}
