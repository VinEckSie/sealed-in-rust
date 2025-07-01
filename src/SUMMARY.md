📘 Sealed in Rust — Table of Contents (Domain-Driven Edition)

- [Introduction](introduction.md)
- [How to Use This Book](how-to-use.md)  

---

# Part 1 — Why Rust Meets Cryptography
- [1.1. Cryptography is a Systems Problem](01-foundations/01-01-cryptography.md)
- [1.2. Safety, Performance, Predictability](01-foundations/01-02-rust-offers.md)
- [1.3. Cost of Unsafety: Famous Failures](01-foundations/01-03-cost-unsafety.md)
- [1.4. First Code: A Naive XOR Encryptor](01-foundations/01-04-first-code.md)
- [1.5. Tooling Up](01-foundations/01-05-tooling-up.md)

<!--
# Part 2 — Core Cryptographic Primitives
Each chapter includes:
Intuition & mental models
Rust crate ecosystem
Safe APIs and dangerous pitfalls
- [2.1. Symmetric Ciphers — XOR, AES, ChaCha20]()
- [2.2. Cryptographic Hashes — SHA-2, BLAKE3]()
- [2.3. MACs & AEAD — HMAC, Poly1305, AES-GCM]()
- [2.4. Public-Key Cryptography — DH, RSA, ECC]()
- [2.5. Digital Signatures — RSA, Ed25519, ECDSA]()
- [2.6. Key Derivation Functions — Argon2, scrypt]()
- [2.7. Randomness & Entropy — Nonces, IVs, CSPRNGs]()
- [2.8. Merkle Trees & Hash Chains — For integrity & proofs]()
- [2.9. Zero-Knowledge Proofs (ZKPs) — Intros, circuits, tooling]()
- [2.10. Post-Quantum Crypto — Kyber, Dilithium]()

# Part 3 — Applied Domains for Secure Systems
[Intro]()
Each chapter explains:
Threat model
Real-world needs
Rust architecture
How crypto primitives are applied

[🔐 Identity & Access Systems]()
🥇 Most universal, job-relevant, and Rust-friendly.
Covers passwordless login (WebAuthn), JWTs, OAuth/OIDC, secure tokens, session encryption, and key management.
📦 Crates: jsonwebtoken, argon2, webpki, ring

[🔧 Secure Infrastructure]()
🥈 High usability, backend dev friendly, broad scope.
Topics: encrypted storage, password vaults, file integrity, E2E comms, secure CI/CD secrets handling.
📦 Crates: aes-gcm, blake3, age, secrecy

[🪙 Blockchain & Web3]()
🥉 Massive Rust ecosystem (Substrate, ZK), proven job vector.
Covers wallets, keypairs, Merkle proofs, secp256k1, zk-SNARKs/STARKs, smart contract verification.
📦 Crates: k256, ed25519-dalek, bellman, halo2

[🚘 Automotive, IoT & Embedded]()
✅ Rust's sweet spot: low-level + safety-critical.
Topics: firmware signing, OTA updates, secure ECUs, constrained crypto (PRESENT, Ascon).
📦 Crates: ascon, p256, tinyvec, heapless

[🛰 Defense & Aerospace]()
✅ Rust is gaining adoption in high-assurance systems.
Topics: tamper resistance, telemetry integrity, real-time secure comms, supply chain trust.
📦 Crates: sha2, aes, getrandom, ed25519

[🧾 Government & Voting]()
⚠️ Niche but high-credibility and technically rich.
Topics: verifiable elections, end-to-end auditable systems, blind signatures.
📦 Crates: rsa, ed25519, pairing, zkp toolkits

[🧬 Medical & Health Systems]()
⚠️ Important for ethics and regulation; often overlooked.
Topics: EMR encryption, consent proofs, audit logging, HIPAA-aligned storage.
📦 Crates: aes-gcm, hmac, blake3, secrecy

[🧠 AI & Privacy-preserving ML]()
🔥 Forward-looking, research-heavy, strong Rust edge.
Topics: federated learning, ZK-friendly proofs, differential privacy, encrypted inference.
📦 Crates: snarkvm, prive, circom-rs

[🧪 Offensive Security & Red Team Tooling]()
🧠 Very technical, niche but high value in devsecops.
Topics: C2 channels, encrypted payloads, obfuscation, protocol fuzzing.
📦 Crates: rc4, xor, chacha20, criterion, cargo-fuzz

# Part 4 — Build Your Own Crypto Crate
Architecting Safe-by-Design APIs

Modular Code for Reuse & Audit

Testing: Unit, Property, Fuzzing

Documentation: From README to doc comments

Publishing & Maintaining a Secure Rust Crate

# Appendices
A. Glossary: Rust & Crypto Terminology

B. Crypto Crates by Use Case

C. Suggested Books, Papers & Videos

D. sealed-in-rust Code Repo Map

E. Project Challenges by Difficulty


-----------

[Contributors](misc/contributors.md)

-->