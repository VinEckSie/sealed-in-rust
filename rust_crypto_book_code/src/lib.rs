// ANCHOR: aes
use aes::Aes128;
use block_padding::Pkcs7;
use cbc::{Encryptor, Decryptor};
use cipher::{BlockEncryptMut, BlockDecryptMut, KeyIvInit};

pub fn run_aes_example() {
    let key = b"verysecretkey123";
    let iv = b"uniqueinitvector";
    let plaintext = b"Attack at dawn!";

    let mut buffer = plaintext.to_vec();
    let pos = buffer.len();
    buffer.resize(pos + 16, 0u8);

    let encryptor = Encryptor::<Aes128>::new(key.into(), iv.into());
    let ciphertext = encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
        .expect("encryption failure");

    println!("Ciphertext (hex): {}", hex::encode(ciphertext));

    let decryptor = Decryptor::<Aes128>::new(key.into(), iv.into());

    let mut ciphertext_buffer = ciphertext.to_vec();
    let decrypted = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut ciphertext_buffer)
        .expect("decryption failure");

    println!("Decrypted text: {}", String::from_utf8_lossy(decrypted));
    assert_eq!(plaintext.to_vec(), decrypted);
}
// ANCHOR_END: aes

// ANCHOR: chacha20
pub fn run_chacha20_example() {
    use chacha20::cipher::{KeyIvInit, StreamCipher};
    use chacha20::ChaCha20;

    let key = *b"an example very very secret key!";
    let nonce = *b"unique nonce";
    let plaintext = b"Secret meeting at midnight".to_vec();

    let mut ciphertext = plaintext.clone();

    let mut encryptor = ChaCha20::new(&key.into(), &nonce.into());
    encryptor.apply_keystream(&mut ciphertext);

    println!("Ciphertext (hex): {}", hex::encode(&ciphertext));

    let mut decrypted = ciphertext.clone();
    let mut decryptor = ChaCha20::new(&key.into(), &nonce.into());
    decryptor.apply_keystream(&mut decrypted);

    println!("Decrypted text: {}", String::from_utf8_lossy(&decrypted));
    assert_eq!(plaintext, decrypted);
}
// ANCHOR_END: chacha20
