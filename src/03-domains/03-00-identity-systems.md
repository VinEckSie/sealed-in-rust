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
{{#include ../../rust_crypto_book_code/examples/domain-identity-00-argon2.rs}}
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


[^blast-radius]: Blast radius: the scope of impact when something fails in a system. It describes how much of the system is affected by a bug, outage, security breach, or bad deployment. Goal in engineering: keep the blast radius as small as possible so failures stay contained. [More](../99-appendices/99-01-glossary.md#blast-radius)
