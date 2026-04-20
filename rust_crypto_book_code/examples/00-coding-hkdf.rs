use hkdf::Hkdf;
use sha2::Sha256;

fn main() {
    let salt: [u8; 16] = *b"unique-salt-1234";
    let ikm: [u8; 32] = *b"0123456789ABCDEF0123456789ABCDEF";

    let hk = Hkdf::<Sha256>::new(Some(&salt), &ikm);
    let mut okm = [0u8; 32];

    hk.expand(b"encryption key", &mut okm).unwrap();

    println!("OKM: {:02x?}", okm);
}
