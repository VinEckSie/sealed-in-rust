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

    let mut encryptor = Encryptor::<Aes128>::new(key.into(), iv.into());
    let ciphertext = encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
        .expect("encryption failure");

    println!("Ciphertext (hex): {}", hex::encode(ciphertext));

    let mut decryptor = Decryptor::<Aes128>::new(key.into(), iv.into());

    let mut ciphertext_buffer = ciphertext.to_vec(); // make it mutable
    let decrypted = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut ciphertext_buffer)
        .expect("decryption failure");


    println!("Decrypted text: {}", String::from_utf8_lossy(decrypted));
    assert_eq!(plaintext.to_vec(), decrypted);
}
// ANCHOR_END: aes

