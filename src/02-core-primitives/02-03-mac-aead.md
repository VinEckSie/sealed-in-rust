
## MACs & AEAD — HMAC, Poly1305, AES-GCM

> 🔐 **Used in:** TLS, APIs, cookies, tokens, VPNs, messaging apps
>
> ✅ Essential for integrity and authenticity, not optional.

Encryption hides data.
But encryption alone does not stop attackers from modifying it.

Modern cryptography requires two guarantees:

- **Confidentiality** → Nobody can read the data
- **Integrity & Authenticity** → Nobody can tamper with it unnoticed

This chapter is about the second half — the part beginners forget, and attackers exploit.

## Why Encryption Alone Is Not Enough

A common beginner mistake:
“My data is encrypted, so it’s secure.”
That’s false.

If an attacker can flip bits in your ciphertext and you don’t detect it, your system is broken.

Real consequences:
- Modified database records
- Forged API requests[^forged-api]
- Token manipulation[^token-manip]
- Padding-oracle exploits[^padding-oracle]
- Silent data corruption[^silent-data]

**Encryption without integrity is malleable by default**.

This is why **MACs** and **AEAD** exist.

## MACs (Message Authentication Codes)

A MAC is a cryptographic checksum computed using a secret key.

It answers one critical question:

> “Was this message produced by someone who knows the secret key — and was it modified?”

MACs provide:
- Integrity (detect modification)
- Authenticity (prove origin)
- No confidentiality (data remains readable)

A MAC is not encryption.

```text
message + secret key → MAC
```

How to verify:
```text
message + secret key → recomputed MAC → compare
```

If even one bit changes, verification fails.

## HMAC (Hash-based Message Authentication Code) — The Standard MAC Construction

> 💡 Used in JWT[^jwt], APIs[^api], OAuth[^oauth], AWS signing[^aws-signin], TLS internals[^tls-internals]
>
> Stable, conservative, battle-tested

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [hmac](https://crates.io/crates/hmac)

HMAC allows two parties sharing a secret key to authenticate a message and detect any tampering, without requiring encryption.

HMAC combines:
- a cryptographic hash function (e.g. SHA-256)
- a secret key
- a hardened construction resistant to length-extension attacks[^length-attack]

Unlike naïve `hash(key || message)`, HMAC is safe.

🧪 **Code Example: HMAC-SHA256** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:hmac}}
```

If the message or tag is altered, verification fails immediately.

> 🚨 **Critical rule**
> : Never compare MACs with `==`.
> Always use constant-time verification APIs.

> **🟢 Conclusion**
>
> HMAC is conservative, widely deployed, and extremely hard to misuse.
>
> If you need integrity without encryption — HMAC is the right tool.


## Poly1305 — One-Time MAC for Modern Crypto

> 💡 Used in ChaCha20-Poly1305, TLS 1.3, WireGuard
>
> Extremely fast, simple, and timing-safe

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [poly1305](https://crates.io/crates/poly1305)

Poly1305 is a modern MAC designed by Daniel J. Bernstein.

Key properties:

* One-time MAC (key must never be reused)
* Constant-time[^constant-time] by design
* Very small and fast
* Designed to pair with stream ciphers[^stream-cipher]

Poly1305 is almost never used alone.
It is generated from a cipher keystream, usually ChaCha20.

🧪 **Code Example: Poly1305** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:poly1305}}
```

> 🚨 **Critical rule**
> : Poly1305 keys must never be reused.
> Reuse = forgery.

> **🟢 Conclusion**
>
> Poly1305 is fast, elegant, and extremely secure when used correctly,
> but it must be paired with a cipher that guarantees fresh keys.

## AEAD — Authenticated Encryption (The Right Way)

Modern cryptography does not ask: “Should I encrypt or authenticate?”.

The answer is: **Both. Together. Always.**

AEAD guarantees:

* Confidentiality
* Integrity
* Authenticity
* Optional authentication of unencrypted metadata

If authentication fails → decryption must not happen.

## AES-GCM — The Enterprise Standard AEAD

> 💡 Used in TLS, HTTPS, databases, cloud storage, hardware security modules, hardware-accelerated and widely standardized

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crates used: [aes-gcm](https://crates.io/crates/aes-gcm)

AES-GCM[^aes-gcm] combines:

* AES block cipher[^aes-block-cipher]
* CTR mode[^ctr-mode] (for encryption)
* GHASH[^ghash]  (for authentication)

🧪 **Code Example: AES-256-GCM** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))


```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:aes256gcm}}
```

Output:
```text
AES256-GCM: [
    115,
    101,
    99,
    114,
    101,
    116,
    32,
    100,
    97,
    116,
    97,
]
```

> 🚨 **Critical rule**
> : Never reuse a nonce with the same key. Ever.
> GCM nonce reuse = total compromise.

> **🟢 Conclusion**
>
> AES-GCM is extremely fast on modern CPUs and ideal for servers,
> but nonce management must be flawless.


## ChaCha20-Poly1305 — The Safer Default

> 💡 Used in WireGuard[^wireguard], mobile apps, embedded systems, TLS fallback[^tls-fallback]
>
> Designed for misuse resistance[^misuse-resistance]

ChaCha20-Poly1305[^chacha20-poly1305] combines:

* ChaCha20[^chacha20] (encryption)
* Poly1305[^poly1305] (authentication)
* A clean, unified AEAD API[^aead-api]

Advantages:

* Constant-time[^constant-time] by design
* No cache-timing[^cache-timing] issues
* Excellent performance everywhere
* Fewer catastrophic mistakes


🧪 **Code Example: ChaCha20-Poly1305** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))


```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:chacha20poly1305}}
```

Output:
```text
Chacha20-Poly1305: [
    115,
    101,
    99,
    114,
    101,
    116,
    32,
    100,
    97,
    116,
    97,
]
```

> **🟢 Conclusion**
>
> ChaCha20-Poly1305 is often the best default choice: safer APIs, portable performance, and strong resistance to side-channel attacks[^side-channel-attack].


## MAC vs AEAD — What Should You Use?

| Situation          | Use                      |
| ------------------ | ------------------------ |
| Integrity only     | HMAC                     |
| Streaming cipher   | Poly1305 (with ChaCha20) |
| General encryption | AEAD                     |
| Enterprise systems | AES-GCM                  |
| Mobile / embedded  | ChaCha20-Poly1305        |

> 🚨 **Critical rule**
> : If encryption is involved → always use AEAD.
> Rolling your own MAC + encryption is a mistake.

> **🟢 Conclusion**
>
> Encryption alone only hides data — it doesn’t stop attackers from changing it.
>
> A MAC teaches the integrity lesson: “was this message modified, and did it come from someone with the key?”
>
> Poly1305 shows modern MAC design: fast and safe when used correctly (one-time keys).
>
> AEAD (AES-GCM, ChaCha20-Poly1305) combines encryption + authentication so tampering is detected and decryption is refused.
>
> Practical rule: if you encrypt data in real systems, **use AEAD by default** and treat nonce/key management as a first-class security requirement.


[^forged-api]: Forged API Request — Attack where an adversary crafts or alters API requests to impersonate a legitimate client or bypass authentication and authorization controls. [More](../99-appendices/99-01-glossary.md#forged-api-request)
[^token-manip]: Token Manipulation — Tampering with authentication or session tokens (JWT, cookies, API keys) to escalate privileges, extend validity, or impersonate another user. [More](../99-appendices/99-01-glossary.md#token-manipulation)
[^padding-oracle]: Padding Oracle Attack — Cryptographic attack exploiting padding validation errors in block ciphers to progressively recover plaintext or forge valid ciphertexts. [More](../99-appendices/99-01-glossary.md#padding-oracle-attack)
[^silent-data]: Silent Data Corruption — Undetected modification of data caused by hardware faults, software bugs, or transmission errors, leading to integrity loss without immediate failure signals. [More](../99-appendices/99-01-glossary.md#silent-data-corruption)
[^jwt]: JWT (JSON Web Token) — Compact, URL-safe token format used to securely transmit signed or encrypted claims for authentication and authorization in distributed systems. [More](../99-appendices/99-01-glossary.md#jwt)
[^api]: API (Application Programming Interface) — Contract defining how software components communicate via structured requests, responses, authentication, and versioned endpoints. [More](../99-appendices/99-01-glossary.md#api)
[^oauth]: OAuth 2.0 — Industry-standard authorization framework enabling delegated access to protected resources without sharing user credentials with third-party applications. [More](../99-appendices/99-01-glossary.md#oauth-20)
[^aws-signin]: AWS Request Signin — Cryptographic mechanism (SigV4) that authenticates and authorizes AWS API requests using HMAC-based signatures derived from secret credentials. [More](../99-appendices/99-01-glossary.md#aws-request-signin-sigv4)
[^tls-internals]: TLS Internals — Cryptographic protocols and handshake mechanisms that provide authentication, key exchange, confidentiality, and integrity for secure network communications. [More](../99-appendices/99-01-glossary.md#tls-internals)
[^length-attack]: Length-Extension Attack — Hash function vulnerability allowing attackers to append data to a hashed message and compute a valid hash without knowing the secret. [More](../99-appendices/99-01-glossary.md#length-extension-attack)
[^constant-time]: Constant-Time Algorithm — Implementation strategy where execution time is independent of secret data, preventing timing side-channel information leakage. [More](../99-appendices/99-01-glossary.md#constant-time-algorithm)
[^stream-cipher]: Stream Cipher — Symmetric encryption primitive that generates a pseudorandom keystream and encrypts data by XORing it with plaintext bytes. [More](../99-appendices/99-01-glossary.md#stream-cipher)
[^aead]: AEAD — Encryption that also authenticates ciphertext and optional associated data (AAD), producing a tag; decryption fails if verification fails. [More](../99-appendices/99-01-glossary.md#aead)
[^aes-gcm]: AES-GCM — Standard AEAD using AES-CTR for encryption plus GHASH for authentication; fast but requires unique nonces per key. [More](../99-appendices/99-01-glossary.md#aes-gcm)
[^aes-block-cipher]: AES Block Cipher — AES is a 128-bit block cipher primitive; you still need a mode/AEAD (CTR, GCM, etc.) to encrypt messages safely. [More](../99-appendices/99-01-glossary.md#aes-block-cipher)
[^ctr-mode]: CTR Mode — Counter mode turns a block cipher into a keystream generator; it’s fast but malleable and must be paired with authentication. [More](../99-appendices/99-01-glossary.md#ctr-mode)
[^ghash]: GHASH — The polynomial-hash authenticator inside AES-GCM, computed over AAD and ciphertext to help produce the authentication tag. [More](../99-appendices/99-01-glossary.md#ghash)
[^wireguard]: WireGuard — Modern VPN protocol using a small, fixed set of strong primitives (notably ChaCha20-Poly1305) for high performance and simplicity. [More](../99-appendices/99-01-glossary.md#wireguard)
[^tls-fallback]: TLS Fallback — Choosing an alternative cipher suite during TLS negotiation (often ChaCha20-Poly1305 on non-AES hardware) without sacrificing authenticated encryption. [More](../99-appendices/99-01-glossary.md#tls-fallback)
[^misuse-resistance]: Misuse Resistance — Design goal where common mistakes (especially nonce reuse) cause less catastrophic failure; still not “safe to misuse,” just safer. [More](../99-appendices/99-01-glossary.md#misuse-resistance)
[^chacha20]: ChaCha20 — Fast ARX-based stream cipher; encrypts by XORing a generated keystream with plaintext; nonce reuse under the same key breaks confidentiality. [More](../99-appendices/99-01-glossary.md#chacha20)
[^poly1305]: Poly1305 — One-time MAC producing a 16-byte tag; secure only if the Poly1305 key is never reused, typically derived fresh per message. [More](../99-appendices/99-01-glossary.md#poly1305)
[^chacha20-poly1305]: ChaCha20-Poly1305 — Widely deployed AEAD combining ChaCha20 encryption with Poly1305 authentication; strong, fast in software, and common in TLS and WireGuard. [More](../99-appendices/99-01-glossary.md#chacha20-poly1305)
[^aead-api]: AEAD API — Standard encrypt/decrypt interface using (key, nonce, AAD, plaintext/ciphertext) and returning success or authentication failure; never release plaintext on failure. [More](../99-appendices/99-01-glossary.md#aead-api)
[^cache-timing]: Cache Timing — Timing leakage caused by secret-dependent CPU cache accesses (e.g., lookup tables), which can reveal keys; mitigated by constant-time code and AES-NI. [More](../99-appendices/99-01-glossary.md#cache-timing)
[^side-channel-attack]: Side-Channel Attack — Attack that exploits leaked information from an implementation (timing, cache, power, EM, faults) rather than breaking the underlying cryptography. [More](../99-appendices/99-01-glossary.md#side-channel-attack)
