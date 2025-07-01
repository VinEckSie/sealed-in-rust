# Summary

[Introduction](README.md)

---
# User Guide

- [Installation](guide/installation.md)
- [Reading Books](guide/reading.md)
- [Creating a Book](guide/creating.md)


---
# PART 1 — Foundations
- [Introduction to Cryptography & Rust]()
- [What is cryptography?]()
- [What makes Rust a good fit?]()
- [The importance of correctness & memory safety]()
- [The Language of Secrets: Basic Terminology]()
- [Confidentiality, integrity, authenticity]()
- [Adversary models, threat models, attacker capabilities]()
- [Math Primer for Cryptographers]()
- [Modular arithmetic]()
- [Prime numbers & GCD]()
- [Groups, rings, fields (intro only, not academic overkill)]()


# PART 2 — Classical Ciphers & First Code
- [From Caesar to Vigenère: Classical Algorithms]()
- [Historical context + Rust implementation]()
- [Frequency analysis]()
- [Code: cracking Caesar & substitution ciphers]()
- [Modern Attacks on Classical Ciphers]()
- [Why these are broken]()
- [Key reuse, predictability]()
- [Transition to modern crypto]()

# PART 3 — Symmetric Cryptography
- [The Magic of XOR]()
- [Properties]()
- [One-Time Pad]()
- [Rust: custom XOR stream cipher]()
- [Block Ciphers: Building Blocks]()
- [Feistel networks, Substitution-Permutation Networks]()
- [Code: simplified DES / mini-AES for illustration]()
- [AES and Block Cipher Modes]()
- [ECB, CBC, CTR, GCM]()
- [Rust: aes, cipher, block-modes crates]()
- [Code: encrypt/decrypt with proper mode handling]()
- [Authenticated Encryption]()
- [AEAD explained]()
- [Rust: aes-gcm, chacha20poly1305]()
- [Code: secure file/message encryption]()

# PART 4 — Hashing & Integrity
- [Cryptographic Hash Functions]()
- [Properties: preimage, collision resistance]()
- [Rust: sha2, blake3]()
- [Code: password hashing, file integrity checker]()
- [Message Authentication Codes (MACs)]()
- [HMAC, CMAC]()
- [Rust: hmac, crypto-mac]()
- [Code: simple API message verifier]()

# PART 5 — Asymmetric Cryptography
- [Public Key Cryptography 101]()
- [Key pairs, trapdoor functions]()
- [Diffie-Hellman explained visually]()
- [RSA: Theory and Rust]()
- [Math background (modular exponentiation, totients)]()
- [Code: DIY RSA in Rust, then compare to crate]()
- [Elliptic Curve Cryptography]()
- [Why ECC? High-level curve intuition]()
- [Rust: curve25519-dalek, p256]()
- [Code: generate and exchange keys]()
- [Digital Signatures]()
- [RSA & ECDSA signatures]()
- [Rust: ed25519-dalek, rsa crate]()
- [Code: verify and sign documents]()

# PART 6 — Applications & Design
- [Key Derivation Functions]()
- [KDFs, PBKDF2, scrypt, Argon2]()
- [Rust: argon2, ring]()
- [Code: password-based key generation]()
- [Randomness & Entropy]()
- [Secure RNGs, nonces, IVs]()
- [Rust: rand_core, getrandom, ring]()
- [Secure Design Principles]()
- [Don't roll your own crypto]()
- [Common pitfalls]()
- [Rust-specific safety techniques]()
- [18.5 Merkle Trees & Blockchain Primitives]()
- [Build a Merkle Tree in Rust]()
- [Use SHA-256, Keccak]()
- [Explain transaction integrity]()
- [secp256k1 and Crypto in Blockchain]()
- [Key generation, message signing]()
- [Crates: k256, libsecp256k1]()

# PART 7 — Advanced Topics
- [Zero-Knowledge Proofs (Intro)]()
- [What they are, why they matter]()
- [Link to zk-SNARKS, but no deep dive]()
- [Post-Quantum Cryptography]()
- [Lattices, NTRU, Kyber]()
- [Rust: reference Kyber implementation overview]()
- [20. Zero-Knowledge Proofs (Deep Dive)]()
- [ zk-SNARKs vs zk-STARKs]()
- [Tools: bellman, circom, halo2]()
- [21. Post-Quantum Cryptography]()
- [    Kyber, Dilithium, Falcon]()
- [    Rust: pqcrypto, kyber crate]()
- [22. Lightweight Cryptography for Embedded]()
- [    Ascon, PRESENT]()
- [    Code: implement toy cipher with embedded constraints]()
- [23. Side-Channel Attacks & Constant-Time Rust]()
- [    Code: benchmark constant-time XOR vs unsafe one]()
- [    Tools: criterion, valgrind]()
- [24. Cryptography for Secure Boot & Firmware]()
- [    Overview: signature chains]()
- [    Code: simulate signed firmware verification]()

# PART 8 — Your Own Crypto Crate
- [Designing Your First Crypto Crate]()
- [Architecture]()
- [Docs, tests, benchmarks]()
- [Code: A mini symmetric cipher with Rust idioms]()
- [Fuzzing, Auditing & Publishing]()
- [cargo-fuzz, Clippy, criterion]()
- [Audit checklists]()
- [Crate publishing & security considerations]()

# Appendices
- [A. Glossary of Terms]()
- [B. List of Rust Crypto Crates (by category)]()
- [C. Suggested Reading & Further Learning]()
- [D. Crate: rust_crypto_book_code structure]()


- [Nested example](nested/README.md)
- [Subchapter](nested/sub-chapter.md)
---
# Summary2
- [First Chapter](relative/path/to/markdown.md)
- [Second Chapter](relative/path/to/markdown2.md)

- [Draft Chapter]()

- [My First Chapter](my-first-chapter.md)

-----------

[Contributors](misc/contributors.md)