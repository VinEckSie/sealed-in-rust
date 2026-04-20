use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

fn sign_claims(message: &[u8]) -> (SigningKey, VerifyingKey, Signature) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let signature = signing_key.sign(message);

    (signing_key, verifying_key, signature)
}

fn verify_claims(message: &[u8], verifying_key: &VerifyingKey, signature: &Signature) -> bool {
    verifying_key.verify(message, signature).is_ok()
}

fn main() {
    let claims = br#"{"sub":"alice","role":"admin","exp":1735689600}"#;

    let (_signing_key, verifying_key, signature) = sign_claims(claims);

    let valid = verify_claims(claims, &verifying_key, &signature);
    println!("Valid signature: {valid}");

    let tampered_claims = br#"{"sub":"alice","role":"super_admin","exp":1735689600}"#;
    let tampered_valid = verify_claims(tampered_claims, &verifying_key, &signature);
    println!("Tampered claims accepted: {tampered_valid}");
}
