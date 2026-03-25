use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;
use rsa::rand_core;

fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, stored_hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let parsed_hash = PasswordHash::new(stored_hash)?;
    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = "correct horse battery staple";

    let stored_hash = hash_password(password)?;
    println!("Stored hash:\n{stored_hash}\n");

    let valid = verify_password(password, &stored_hash)?;
    println!("Valid password: {valid}");

    let invalid = verify_password("wrong password", &stored_hash)?;
    println!("Wrong password accepted: {invalid}");

    Ok(())
}
