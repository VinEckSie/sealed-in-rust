📘 Sealed in Rust — Table of Contents (Domain-Driven Edition)

- [Introduction](00-Intro/introduction.md)
- [How to Use This Book](00-Intro/how-to-use.md)

---

- [Why Rust Meets Cryptography]()
    - [Cryptography is a Systems Problem](01-foundations/01-01-cryptography.md)
    - [Safety, Performance, Predictability](01-foundations/01-02-rust-offers.md)
    - [Cost of Unsafety: Famous Failures](01-foundations/01-03-cost-unsafety.md)
    - [First Code: A Naive XOR Encryptor](01-foundations/01-04-first-code.md)
    - [Tooling Up](01-foundations/01-05-tooling-up.md)
<!-- Cryptographic Engineering & Failure Modes -->
<!-- Side-Channel Attacks & Constant-Time Design -->

- [Core Cryptographic Primitives]()
    - [How Ciphers, Hashes, MACs, AEAD Compose](02-core-primitives/02-00-primitives-overview.md)
    - [Symmetric Ciphers — XOR, AES, ChaCha20](02-core-primitives/02-01-symmetric-ciphers.md)
    - [Cryptographic Hashes — SHA-2, BLAKE3](02-core-primitives/02-02-crypto-hashes.md)
    - [MACs & AEAD — HMAC, Poly1305, AES-GCM](02-core-primitives/02-03-mac-aead.md)
    - [Key Derivation Functions — Argon2, scrypt](02-core-primitives/02-06-key-derivation.md)
    - [Randomness & Entropy — Nonces, IVs, CSPRNGs](02-core-primitives/02-07-randomness-entropy.md)

- [Appendices]()
    - [Glossary of Terms](99-appendices/99-01-glossary.md)
    - [Cryptographic Concepts Cheatsheet](99-appendices/99-02-cheatsheet.md)
