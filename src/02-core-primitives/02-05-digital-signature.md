## Digital Signatures: Ed25519, ECDSA, RSA

> 🔐 **Used in:** TLS certificates, SSH, software updates, package signing, blockchains, identity systems
>
> ✅ A digital signature is public proof that *this exact message* came from the holder of a private key.

Digital signatures solve a problem encryption does not:

- Encryption answers: “Who can read this?”
- Signatures answer: “Who wrote this, and was it changed?”

If you can verify a signature, you can trust:
- Authenticity: the signer had the private key
- Integrity: the message was not modified

That is why signatures are the backbone of:
- certificate chains[^certificate-chain]
- secure updates
- authenticated key exchange[^authenticated-key-exchange] (e.g., TLS handshake signatures)
- “verify before you run” security[^verify-before-run-security]

## The Basic Model: sign → verify

In almost every signature scheme, you can think like this:

```text
signature = Sign(private_key, message)
ok = Verify(public_key, message, signature)
```

Anyone can verify.
Only the private-key holder can sign.

## Hash-Then-Sign (why hashes show up everywhere)

Signatures are almost never computed over “big messages” directly.

Instead, protocols use:

```text
digest = Hash(message)
signature = Sign(private_key, digest)
```

Why?
- Hashes are fixed-size (fast to sign, easy to transport)
- Hashes bind the *entire* message (one-bit change → totally different digest)
- Collision resistance[^collision] matters: signatures inherit the security of the hash

This is why you’ll constantly see “SHA-256 + ECDSA” or “SHA-256 + RSA-PSS”.

## Ed25519: modern default for new systems

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [ed25519-dalek](https://crates.io/crates/ed25519-dalek)

Ed25519 is an elliptic-curve[^elliptic-curve] signature scheme designed to be:
- fast
- hard to misuse
- deterministic (no fragile per-signature randomness like classic ECDSA)

The name “Ed25519” refers to an Edwards-curve signature scheme using the Curve25519 family with a 255-bit prime field.

It’s an excellent default when you control both ends of a system (new protocol, internal services, new products).

🧪 **Code Example: Ed25519** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs#L227))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:ed25519}}
```

Output:
```text
Ed25519 signature verified.
```

> 🚨 **Critical rule:**
> Always verify signatures with the library’s verification API (constant-time comparisons[^constant-time-comparison] and correct checks[^correct-check]).
> Never compare signatures manually.

## ECDSA: widely deployed, easy to get wrong

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [p256](https://crates.io/crates/p256)

ECDSA is the “classic” elliptic-curve signature family used everywhere (TLS, hardware tokens, many enterprise systems).
ECDSA stands for Elliptic Curve Digital Signature Algorithm (DSA adapted to elliptic curves).

The danger in ECDSA is the per-signature secret nonce `k`:

- If `k` is ever reused → the private key can be recovered
- If `k` is biased/predictable → the private key can leak

Many libraries use deterministic nonces[^deterministic-nonce] (RFC 6979) to reduce RNG foot-guns[^rng-foot-gun], but the easiest safe rule is: use well-audited libraries and don’t customize nonce generation.

🧪 **Code Example: ECDSA** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs#L243))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:ecdsa}}
```

Output:
```text
ECDSA P-256 signature verified.
```

## RSA signatures: compatibility workhorse (use RSA-PSS)

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [rsa](https://crates.io/crates/rsa)

RSA is older than ECC[^ECC], but it’s still deeply embedded in the world (certificate ecosystems, legacy devices, enterprise).

Important RSA signature reality:
- Raw RSA is not a signature scheme
- You must use a padding scheme designed for signatures

Today, that means:
- Prefer RSA-PSS[^rsa-pss] (modern, safer)
- Avoid “textbook RSA”[^textbook-rsa] entirely

🧪 **Code Example: RSA** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs#L261))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:rsa}}
```

Output:
```text
RSA-PSS signature verified.
```

## Which one should you choose?

If you’re designing something new: Ed25519 is usually the best default

If you need broad TLS[^broad-tls] / enterprise / hardware ecosystem compatibility: ECDSA (P-256) is common

If you must support legacy systems or existing PKI deployments[^pki-deployment]: RSA (prefer PSS) still shows up everywhere

## Common Mistakes (how signatures fail in real life)

- **Confusing signatures with encryption**: signing does not hide data; it proves origin and integrity.
- **Verifying the wrong thing**: always verify the exact bytes that were signed (canonical encoding matters).
- **Algorithm confusion**: never accept “any algorithm”; allow-list what you expect (JWT-style mistakes[^jwt] are common)
- **Bad randomness (ECDSA)**: nonces are a single-point-of-failure
- **Using outdated RSA modes**: “RSA without padding” is broken; “RSA-PSS” is the modern baseline

> **🟢 Conclusion**
>
>Digital signatures are the public-key primitive for authenticity.
>Hash-then-sign binds *meaning*, not just bytes.
>
> Ed25519 is the modern default.
>
> ECDSA is everywhere but fragile around nonces.
>
> RSA is still common, but only with the right padding (prefer PSS).

[^certificate-chain]: Certificate chain: ordered set of certificates where each certificate is signed by the next/higher authority up to a trusted root, used to bind a public key to an identity. [More](../99-appendices/99-01-glossary.md#certificate-chain)
[^authenticated-key-exchange]: Authenticated key exchange (AKE): key exchange that also authenticates the peer (via signatures, certificates, or PSKs), preventing active man-in-the-middle attacks. [More](../99-appendices/99-01-glossary.md#authenticated-key-exchange-ake)
[^verify-before-run-security]: Verify-before-run security: practice of verifying a signature on code or artifacts (updates, packages, binaries) before installing or executing them. [More](../99-appendices/99-01-glossary.md#verify-before-run-security)
[^collision]: Collision: two different inputs producing the same hash; collision resistance is critical for signature security. [More](../99-appendices/99-01-glossary.md#collision)
[^elliptic-curve]: Elliptic curve: the mathematical structure used by ECC; curves over finite fields provide the group operations behind ECDSA and Ed25519. [More](../99-appendices/99-01-glossary.md#elliptic-curve)
[^constant-time-comparison]: Constant-time comparison: comparing values (MACs, signatures, hashes) without early exits so timing does not leak information about secret-dependent equality. [More](../99-appendices/99-01-glossary.md#constant-time-comparison)
[^correct-check]: Correct checks (verification hygiene): strict signature verification rules like algorithm allow-lists, parameter validation, and rejecting malformed/malleable signatures. [More](../99-appendices/99-01-glossary.md#correct-checks-signature-verification)
[^deterministic-nonce]: Deterministic nonce: deriving the per-signature nonce from the private key and message (e.g., RFC 6979) to reduce dependence on external randomness at signing time. [More](../99-appendices/99-01-glossary.md#deterministic-nonce-rfc-6979)
[^rng-foot-gun]: RNG foot-gun: common randomness mistake (weak seeding, predictability, reuse) that silently breaks cryptography, especially nonce-based signatures like ECDSA. [More](../99-appendices/99-01-glossary.md#rng-foot-gun)
[^ECC]: ECC (elliptic-curve cryptography): public-key cryptography built on elliptic-curve groups, typically giving smaller keys and faster operations than RSA at comparable security. [More](../99-appendices/99-01-glossary.md#ecc-elliptic-curve-cryptography)
[^rsa-pss]: RSA-PSS: the modern RSA signature padding scheme (“Probabilistic Signature Scheme”), designed to be robust against signature forgeries and recommended over older RSA signature modes. [More](../99-appendices/99-01-glossary.md#rsa-pss)
[^textbook-rsa]: Textbook RSA: raw RSA without a padding scheme; deterministic and malleable, and therefore insecure for both encryption and signatures. [More](../99-appendices/99-01-glossary.md#textbook-rsa)
[^broad-tls]: Broad TLS compatibility: choosing signature algorithms/parameters that work across a wide range of TLS clients, servers, libraries, and devices (often constrained by legacy support). [More](../99-appendices/99-01-glossary.md#broad-tls-compatibility)
[^pki-deployment]: PKI deployment: the real-world operational certificate ecosystem (CAs, issuance policies, revocation, client trust stores) that shapes which algorithms can be used in practice. [More](../99-appendices/99-01-glossary.md#pki-deployment)
[^jwt]: JWT (JSON Web Token): signed token format; secure usage requires strict algorithm allow-lists and proper signature verification. [More](../99-appendices/99-01-glossary.md#jwt)
