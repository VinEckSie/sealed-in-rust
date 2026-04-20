## Domain: Identity & Access Systems

> 🔐 Used in: APIs, SaaS platforms, fintech, internal tools, developer platforms, enterprise SSO, mobile backends
>
> ✅ One of the most practical cryptography domains: combines password hashing, token signing, authenticated sessions, randomness, and secure protocol design.

Identity systems answer two fundamental questions:

- **Who are you?** → authentication
- **What are you allowed to do?** → authorization

Modern identity systems are not built from a single primitive.

They are built from a composition of primitives:

- **Argon2** protects passwords at rest
- **HMAC** protects tokens against tampering
- **Ed25519** enables public verification
- **AEAD** protects confidential session state
- **Randomness** prevents replay and prediction
- **Hashing** enables safe logging and correlation

This chapter shows how those primitives are applied in a real domain.

Not as isolated theory. As a working security architecture.


## Threat Model

Identity systems are constantly attacked.

Attackers try to:

- steal password databases
- brute-force leaked hashes
- forge tokens
- escalate privileges
- replay login challenges
- tamper with session state
- abuse weak randomness
- exploit overexposed long-term secrets



## Identity Systems as Composition of Primitives
Cryptography is not used once. It appears at every layer of the identity system.


| Component                      | Primitive         | Security Property             | Why it matters                      |
| ------------------------------ | ----------------- | ----------------------------- | ----------------------------------- |
| password storage               | Argon2            | brute-force resistance        | database leaks happen               |
| internal stateless token       | HMAC              | integrity + authenticity      | prevents privilege escalation       |
| distributed token verification | Ed25519           | public verifiability          | proves identity                     |
| session protection             | AEAD              | confidentiality + integrity   | protects sensitive state            |
| login challenges               | CSPRNG            | unpredictability              | prevents reuse of captured messages |
| safe observability             | BLAKE3            | non-reversible fingerprinting | limits blast radius[^blast-radius]                 |
| session lifecycle control      | expiration policy | expiration enforcement        | prevents long-lived compromise      |



## Password Storage → Argon2

Passwords must never be stored directly.

If a database leaks and the server stored plaintext passwords, every account is immediately compromised. Even plain hashing is not enough.

General-purpose hashes are designed to be fast. Password hashing must be deliberately expensive.

Argon2 is designed for this exact purpose.

```text
password + salt → Argon2 → password hash
```

This protects users even when the database is stolen.

The server stores only the derived hash, never the password itself.


🧪 **Minimal Rust Example: password hashing and verification** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/examples/01-domain-identity-00-argon2.rs))


> <img src="../images/cargo.png" alt="crate logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crates used: argon2
, rand_core

```rust,no_run
{{#include ../../rust_crypto_book_code/examples/01-domain-identity-00-argon2.rs}}
```

Output:
```text
Stored hash:
$argon2id$v=19$m=19456,t=2,p=1$G1/J1ZCmovy3XioeKSPC1Q$RRoz1mRaShxHqnvpq3JiGR0xuScwdoO6MOcWkUw0cIU

Valid password: true
Wrong password accepted: false
```

> **🟢 Conclusion**
>
>Password hashing solves one specific problem: if the database leaks, raw passwords are not immediately exposed.

## Token Integrity → HMAC

After authentication, the server often issues a token.

That token may contain claims such as:
- user identifier
- role
- expiration time
- scopes

If attackers can modify the token, they can escalate privileges. So the token must be protected against tampering.

A simple and practical way to do that is HMAC.

```text
payload + secret key → MAC
````

When the token comes back, the server recomputes the MAC and checks that the token was not modified.

🧪 **Minimal Rust Example: HMAC-signed token** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/examples/01-domain-identity-01-hmac.rs))


> <img src="../images/cargo.png" alt="crate logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crates used: hmac, sha2, base64

```rust,no_run
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn sign_token(payload: &str, secret: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(payload.as_bytes());

    let tag = mac.finalize().into_bytes();

    let payload_b64 = URL_SAFE_NO_PAD.encode(payload.as_bytes());
    let tag_b64 = URL_SAFE_NO_PAD.encode(tag);

    Ok(format!("{payload_b64}.{tag_b64}"))
}

fn verify_token(token: &str, secret: &[u8]) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let Some((payload_b64, tag_b64)) = token.split_once('.') else {
        return Ok(None);
    };

    let payload = match URL_SAFE_NO_PAD.decode(payload_b64) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };

    let tag = match URL_SAFE_NO_PAD.decode(tag_b64) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };

    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(&payload);

    if mac.verify_slice(&tag).is_ok() {
        Ok(Some(String::from_utf8(payload)?))
    } else {
        Ok(None)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = b"server-secret-key";
    let payload = r#"{"sub":"alice","role":"user","exp":1735689600}"#;

    let token = sign_token(payload, secret)?;
    println!("Token:\n{token}\n");

    let verified = verify_token(&token, secret)?;
    println!("Verified payload: {verified:?}");

    let tampered_token = token.replace("\"user\"", "\"admin\"");
    let verified_tampered = verify_token(&tampered_token, secret)?;
    println!("Tampered token accepted: {verified_tampered:?}");

    Ok(())
}
```

Output:
```text
Token:
eyJzdWIiOiJhbGljZSIsInJvbGUiOiJ1c2VyIiwiZXhwIjoxNzM1Njg5NjAwfQ.Atg477etFsR_F47QcJz1NINk6xLQCi3PEfs_nAectSQ

Verified payload: Some("{\"sub\":\"alice\",\"role\":\"user\",\"exp\":1735689600}")
Tampered token accepted: None
```

> **🟢 Conclusion**
>
>HMAC ensures that a token cannot be modified without detection.
>This is enough for many internal systems where all verifiers can share the same secret.

## Public Verification → Ed25519 Signatures

HMAC works well when one server or a tightly controlled backend verifies everything.

But some identity systems need broader verification:
- API gateways
- microservices
- third-party systems
- federated identity flows
- passwordless authentication

In those systems, sharing one secret with every verifier is a bad idea.

Digital signatures solve this.

```text
payload → Sign(private_key)
```
```text
payload + signature → Verify(public_key)
```

The signer keeps the private key secret.

Everyone else can verify using the public key.


🧪 **Minimal Rust Example: Ed25519 signing and verification** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/examples/01-domain-identity-02-signatures.rs))


> <img src="../images/cargo.png" alt="crate logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crates used: ed25519-dalek, rand_core

```rust, no_run
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;

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
```

Output:
```text
Valid signature: true
Tampered claims accepted: false
```

> **🟢 Conclusion**
>
> Signatures are the right primitive when many parties need to verify identity assertions without sharing signing power.


[^blast-radius]: Blast radius: the scope of impact when something fails in a system. It describes how much of the system is affected by a bug, outage, security breach, or bad deployment. Goal in engineering: keep the blast radius as small as possible so failures stay contained. [More](../99-appendices/99-01-glossary.md#blast-radius)
