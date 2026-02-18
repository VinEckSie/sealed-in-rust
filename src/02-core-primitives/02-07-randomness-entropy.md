## Randomness & Entropy — Nonces, IVs, CSPRNGs

> 🔐 **Used in:** encryption schemes[^encryption-scheme], TLS, digital signatures[^digital-signature], key generation, session tokens
>
> ✅ Unpredictability is security. If randomness fails, everything collapses.

### Why Randomness Is a Security Primitive

Modern cryptography does not just rely on strong mathematics.

It relies on unpredictability.

If an attacker can predict:
- Your encryption IV[^encryption-iv]
- Your nonce[^nonce]
- Your session token
- Your key material

Then your system is broken — even if you use perfect algorithms.

Cryptography without randomness is just math.


## What Is Entropy?

Entropy is a measure of unpredictability.

In cryptography, entropy answers one question:
How hard is it to guess this value?

Examples:

- A 4-digit PIN → ~13 bits of entropy (very weak)
- A 256-bit key → 256 bits of entropy (astronomically strong)

True entropy comes from:
- OS randomness pools[^os-randomness-pool]
- Hardware noise[^hardware-noise]
- Interrupt timing[^interrupt-timing]
- Environmental noise[^environmental-noise]

It does not come from:
- `rand()`[^rand]
- timestamps[^timestamps]
- incremental counters
- predictable seeds[^predicable-seed]


## CSPRNG — Cryptographically Secure Pseudorandom Number Generator

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [rand](https://crates.io/crates/rand)

A CSPRNG[^csprng] is a deterministic algorithm that expands a small amount of true entropy into large amounts of secure random data.

Properties:

- Unpredictable
- Resistant to state recovery
- Backtracking resistant
- Seeded from high-entropy OS sources

Your OS already provides one:
- Linux → `/dev/urandom`
- Windows → CryptGenRandom / BCryptGenRandom
- macOS → SecRandomCopyBytes

Rust exposes this securely through the [`rand` crate](https://crates.io/crates/rand/)

🧪 **Code Example: Secure Random Bytes** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))


```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:csprng}}
```

Output:
```text
32 random bytes: a7d6ba53f00fd0c7a90bb30312cd0b96ae22e1a5f438c91e4d93a62bff35f28d
random u64: 15302970456105725428
```

What you’re seeing is the same OS CSPRNG used in two different ways:

- **32 random bytes**: raw randomness you typically use directly for crypto inputs (keys, nonces, salts, IVs).
- **random u64**: 8 random bytes returned as a convenient numeric type (handy for things like “pick a random index”).

Not predictable. Not reproducible. Not reversible. That’s exactly what we want.


It does not have to be secret.
It must be unique.

Nonces are used in:

- AES-GCM
- ChaCha20-Poly1305
- TLS record encryption[^tls-record-encryption]
- Replay protection systems


> 🚨 **Critical rule:**
> If you ever reuse a nonce with the same key:
> You can completely break the encryption.


> **🟢 Best Practice**
>
>Use 96-bit (12-byte) nonces for GCM
>
>Never reuse them.
>Prefer random nonces unless protocol specifies counters


## What Happens If Randomness Fails?

History is brutal here:
- Debian OpenSSL bug (2008) reduced entropy to 15 bits.
- Android Bitcoin wallet bug reused ECDSA nonces.
- PS3 signing failure reused ECDSA nonce → private key recovered.

In all cases:
- The algorithm was correct.
- Randomness was broken.
- And everything collapsed.


**Most catastrophic crypto failures are entropy failures.**

## Deterministic vs Secure Random

**Never use:**
use rand::thread_rng();

For simulations? Fine.
For cryptography? No.

**Always prefer:**
use rand::rngs::OsRng;

Or use crates that internally rely on secure randomness (like ring, aes-gcm, chacha20poly1305).

## Key Generation — The Most Important Use Case
All cryptographic keys must come from a secure, high-entropy source.

No passwords.
No manual seeds.
No shortcuts.

**Entropy Budget (Thinking Like an Attacker)**

If a value has:
- 32 bits entropy → brute-forceable
- 64 bits entropy → expensive but possible
- 128 bits entropy → practically infeasible
- 256 bits entropy → overkill for most cases

**Modern security baseline: Minimum 128 bits of entropy for long-term security.**

## Randomness in Protocols
Randomness appears everywhere:
- Session IDs
- TLS handshake[^tls-handshake]
- ECDSA nonces[^ecdsa-nonce]
- Key exchange salts[^key-exchange-salt]
- Password salts[^password-salt]
- Token generation
- Secure cookies

You may not see it.
But it is there.

## Common Mistakes
- Seeding RNG manually

- Using timestamps

- Reusing nonces

- Storing IVs incorrectly

- Assuming randomness == secrecy

- Confusing UUID v4 with cryptographic token (context matters)

- Testing Randomness (What You Should NOT Do). Do not test randomness with: “It looks random.”Cryptographic randomness is not visual randomness.

You rely on:

- OS entropy pool
- Well-audited CSPRNG
- Mature cryptographic libraries

**You do not roll your own RNG. Ever.**


> **🟢 Conclusion**
>
>Randomness is not optional.
>It is a foundational cryptographic primitive.
>
>Entropy defines unpredictability. CSPRNG expands secure entropy safely. Nonces must never repeat
>
>IVs must follow algorithm rules
>
>Reused randomness destroys security
>
>Without entropy, cryptography is an illusion.


[^encryption-iv]: Initialization vector (IV): non-secret per-message value required by some cipher modes; must follow scheme rules; reuse can leak information. [More](../99-appendices/99-01-glossary.md#iv-initialization-vector)
[^digital-signature]: Digital signature: private-key proof of authenticity and integrity; anyone with the public key can verify; enables non-repudiation. [More](../99-appendices/99-01-glossary.md#digital-signature)
[^encryption-iv]: Initialization vector (IV): non-secret per-message value required by some cipher modes; must follow scheme rules; reuse can leak information. [More](../99-appendices/99-01-glossary.md#iv-initialization-vector)
[^nonce]: Nonce (“number used once”): per-operation value that must not repeat for a given key; reuse can catastrophically break AEAD security. [More](../99-appendices/99-01-glossary.md#nonce)
[^os-randomness-pool]: OS entropy pool: kernel-managed entropy mixed from multiple sources; used to seed CSPRNGs and provide secure random bytes. [More](../99-appendices/99-01-glossary.md#os-entropy-pool)
[^hardware-noise]: Hardware noise: physical randomness (thermal/electronic noise, oscillator jitter) used as an entropy source for true randomness. [More](../99-appendices/99-01-glossary.md#hardware-noise)
[^interrupt-timing]: Interrupt timing: small unpredictable variations in interrupt arrival times used as entropy input; mixed by the OS for safety. [More](../99-appendices/99-01-glossary.md#interrupt-timing)
[^environmental-noise]: Environmental noise: entropy from external events (user input, device timings, sensors) collected and mixed into the OS pool. [More](../99-appendices/99-01-glossary.md#environmental-noise)
[^rand]: rand: Rust crate for RNG traits and generators; for crypto randomness use OS-backed RNGs like OsRng/getrandom. [More](../99-appendices/99-01-glossary.md#rand-rust-crate)
[^timestamps]: Timestamps: time-derived values are guessable and low-entropy; unsuitable as seeds, keys, nonces, or cryptographic randomness. [More](../99-appendices/99-01-glossary.md#timestamps)
[^predicable-seed]: Predictable seed: RNG seed from guessable inputs (time, counters); makes outputs predictable and can reveal derived secrets. [More](../99-appendices/99-01-glossary.md#predictable-seed)
[^csprng]: CSPRNG: deterministic generator seeded with high entropy; output is computationally indistinguishable from random; resists prediction. [More](../99-appendices/99-01-glossary.md#csprng-cryptographically-secure-pseudorandom-number-generator)
[^tls-record-encryption]: TLS record encryption: per-record symmetric AEAD that encrypts and authenticates application data using traffic keys/nonces. [More](../99-appendices/99-01-glossary.md#tls-record-encryption)
[^tls-handshake]: TLS handshake: negotiates parameters, authenticates peers, and derives shared traffic keys/secrets used for record encryption. [More](../99-appendices/99-01-glossary.md#tls-handshake)
[^ecdsa-nonce]: ECDSA nonce: secret per-signature scalar k; must be unpredictable and never reused; reuse can leak the private key. [More](../99-appendices/99-01-glossary.md#ecdsa-nonce)
[^key-exchange-salt]: Key exchange salt: non-secret random value fed into a KDF/HKDF to separate contexts and reduce key-reuse risks. [More](../99-appendices/99-01-glossary.md#key-exchange-salt)
[^password-salt]: Password salt: unique random value stored with a password hash; prevents rainbow tables and makes identical passwords hash differently. [More](../99-appendices/99-01-glossary.md#password-salt)
