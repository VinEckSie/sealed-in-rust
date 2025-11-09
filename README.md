# 🛡️ Sealed in Rust — Build Security with the Language of Safety

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Book](https://img.shields.io/badge/docs-mdBook-lightgrey.svg)](https://vinecksie.github.io/sealed-in-rust/)
[![Last commit](https://img.shields.io/github/last-commit/VinEckSie/sealed-in-rust)](https://github.com/VinEckSie/sealed-in-rust)

Learn cryptography by coding it in Rust — from **XOR** to **Ed25519**, one primitive at a time.  
No fluff. No math-heavy jargon. Just Rust and security.

## 📖 Overview

**Sealed in Rust** is a hands-on cryptography book and open-source learning project.  
It teaches **cryptographic fundamentals through real Rust code**, bridging theory and engineering for modern, defense-grade systems.

Each chapter focuses on **why it matters** — connecting primitives like XOR, RSA, or HKDF to secure real-world use cases such as authentication, encryption, and telemetry.

**Core Principles**
- Domain-driven understanding of cryptography  
- Practical, implementation-first teaching  
- Defense-grade mindset — no shortcuts, no pseudocode  


## 🌱 The Learning Journey

The book is organized into **four parts**, each designed to bridge **cryptographic theory** with **Rust engineering**.

### Part I — Foundations 🍀
Build your mental model for secure systems:
- Cryptography as a systems problem  
- Rust’s safety guarantees  
- Case studies & mindset (XOR hands-on)

### Part II — Core Primitives 🔐
Implement the building blocks:
- XOR, AES, ChaCha20  
- HMAC, Poly1305  
- RSA, Ed25519, ECDSA  
- Hashes, randomness, ZKPs, post-quantum crypto

### Part III — Real-World Domains 🌍
Apply cryptography in critical sectors:
- Secure Infrastructure  
- Blockchain & Web3  
- Healthcare & Medical Systems  
- Defense & Aerospace  
- Privacy-Preserving AI  
- IoT, Identity, Red Teaming  

### Part IV — Building Secure Rust Crates ⚙️
Engineer security into your own code:
- Safe-by-design APIs  
- Testing (unit, fuzzing, property-based)  
- Documentation & maintainability best practices  


## 📦 Code Companion

Each chapter comes with a practical Rust module in [`Sealed in Rust book code`](https://github.com/VinEckSie/sealed-in-rust-book-code),  
including tests, benchmarks, and reference implementations.

| Module | Topic | Example |
|---------|--------|----------|
| `xor.rs` | Bitwise operations & stream ciphers | `cargo test xor` |
| `feistel.rs` | Block cipher design | `cargo bench feistel` |
| `rsa.rs` | Key generation & encryption | `cargo test rsa` |
| `ed25519.rs` | Digital signatures | `cargo test ed25519` |

All code is self-contained, minimal, and explained line by line in the book.


## 🧱 Built With

- 📚 **mdBook** — for authoring and serving the book  
- 🦀 **Rust** — for all code examples, tests, and benchmarks  
- ⚡ **Criterion.rs** — for performance benchmarking  
- 🧩 **Doctests** — for inline verifiable examples


## 🔬 What to Expect
 - Mental models & safe API design
 - Exploration of top Rust crypto crates
 - Real applied domains (identity, embedded, aerospace, Web3)
 - Modular, auditable, and testable code
 - Progressive chapter releases


## 🧪 Project Status

This project is in progressive release.  
Each chapter is reviewed and benchmarked before publication.  
Focus: readability, correctness, and educational clarity — not production cryptography.


## 📚 Documentation

The rendered version is hosted at:
[👉 Sealed in Rust](https://vinecksie.github.io/sealed-in-rust/)


## 📄 License

Dual-licensed under MIT OR Apache-2.0 — choose either license.


This project is not about writing crypto for crypto’s sake —
it’s about understanding where it fits, and building systems that deserve to be trusted.
