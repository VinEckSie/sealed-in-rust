// ANCHOR: aes
pub fn run_aes_example() {
    use aes::Aes128;
    use cbc::{Decryptor, Encryptor};
    use cipher::block_padding::Pkcs7;
    use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};

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
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

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

// ANCHOR: hkdf
pub fn run_hkdf_example() {
    use hkdf::Hkdf;
    use sha2::Sha256;

    // salt: optional, non-secret, should be random per context/session
    let salt: [u8; 16] = *b"unique-salt-1234";

    // ikm: Input Keying Material (must be high-entropy)
    // In real systems this comes from a handshake secret, a shared key, etc.
    let ikm: [u8; 32] = *b"0123456789ABCDEF0123456789ABCDEF";

    let hk = Hkdf::<Sha256>::new(Some(&salt), &ikm);

    let mut okm = [0u8; 32];
    hk.expand(b"encryption key", &mut okm).unwrap();

    println!("OKM: {:02x?}", okm);
}
// ANCHOR_END: hkdf

// ANCHOR: sha256
pub fn run_sha256_example() {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, b"hello world");

    let result = hasher.finalize();
    println!("SHA-256: {:x}", result);
}
// ANCHOR_END: sha256

// ANCHOR: blake3
pub fn run_blake3_example() {
    use blake3;

    let hash = blake3::hash(b"hello world");
    println!("BLAKE3: {}", hash);
}
// ANCHOR_END: blake3

// ANCHOR: hmac
pub fn run_hmac_example() {
    use hkdf::hmac::Hmac;
    use hkdf::hmac::digest::Mac;
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let key = b"super-secret-key";
    let message = b"transfer=1000&to=alice";

    let mut mac = <HmacSha256 as Mac>::new_from_slice(key).unwrap();
    Mac::update(&mut mac, message);

    let tag = mac.finalize().into_bytes();

    // Verification
    let mut verify = <HmacSha256 as Mac>::new_from_slice(key).unwrap();
    Mac::update(&mut verify, message);
    Mac::verify_slice(verify, tag.as_slice()).unwrap();
}
// ANCHOR_END: hmac

// ANCHOR: poly1305
pub fn run_poly1305_example() {
    use poly1305::{
        Poly1305,
        universal_hash::{KeyInit, UniversalHash},
    };

    let key = [0u8; 32]; // placeholder: must be a one-time key in real use
    let message = b"authenticated message";

    let mut mac = Poly1305::new(&key.into());
    mac.update_padded(message);

    let tag = mac.finalize();
    let _tag_bytes: [u8; 16] = tag.into(); // if plain array wished
}
// ANCHOR_END: poly1305
