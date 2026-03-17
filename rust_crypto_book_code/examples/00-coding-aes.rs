use aes::Aes128;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};

fn main() {
    let key = b"verysecretkey123";
    let iv = b"uniqueinitvector";
    let text = b"Attack at dawn !";

    let mut buffer = text.to_vec();
    let pos = buffer.len();
    buffer.resize(pos + 16, 0u8);

    let encryptor = Encryptor::<Aes128>::new(key.into(), iv.into());

    let ciphertext = encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
        .expect("Encryption failure");

    println!("Ciphertext (hex): {}", hex::encode(ciphertext));

    let decryptor = Decryptor::<Aes128>::new(key.into(), iv.into());
    let mut ciphertext = ciphertext.to_vec();
    let decrypted = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut ciphertext)
        .expect("Decryption failure");

    println!("Decrypted (text): {}", String::from_utf8_lossy(decrypted));
}
