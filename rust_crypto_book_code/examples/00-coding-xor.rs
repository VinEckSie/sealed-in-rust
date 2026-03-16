/// XOR cipher for educational use only.
///
/// # Example
/// ```
/// use rust_crypto_book_code::xor::xor_encrypt;
///
/// let plaintext = b"hello";
/// let key = b"k";
/// let ciphertext = xor_encrypt(plaintext, key);
/// let decrypted = xor_encrypt(&ciphertext, key);
///
/// assert_eq!(plaintext.to_vec(), decrypted);
/// ```
///
pub fn xor_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

fn main() {
    let message = b"hello world";
    let key = b"key";

    let encrypted = xor_encrypt(message, key);
    let decrypted = xor_encrypt(&encrypted, key);

    println!("Encrypted: {:x?}", encrypted);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
}
