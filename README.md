# 🛡️ Sealed in Rust — Applied Cryptography in Rust

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Book](https://img.shields.io/badge/docs-mdBook-lightgrey.svg)](https://vinecksie.github.io/sealed-in-rust/)
[![Last commit](https://img.shields.io/github/last-commit/VinEckSie/sealed-in-rust)](https://github.com/VinEckSie/sealed-in-rust)

Learn cryptography by implementing it in Rust — then understand where and why it matters in real systems.

📘 Landing page → https://vinecksie.super.site/sealed-in-rust  
📖 Book → https://vinecksie.github.io/sealed-in-rust/


# Approach

Sealed in Rust focuses on two complementary dimensions.

## 1. Primitives
Understand how cryptographic building blocks work:

hashing • symmetric encryption • signatures • MAC • AEAD • KDF • randomness

Each primitive is introduced through:
- minimal Rust implementations
- engineering intuition
- safe usage patterns


## 2. Domains
Understand where cryptography is used in real systems:

identity • authentication • secure communication  
distributed systems • embedded environments  
data integrity • protocol design  

Focus is placed on how primitives combine to support reliable software.


# Current Topics

Implemented or in progress:

- entropy, randomness, CSPRNG
- hashing (SHA-2, BLAKE3)
- AES and ChaCha20
- HMAC, Poly1305, AEAD constructions
- key derivation functions
- digital signatures (Ed25519)
- protocol design foundations
- password storage and identity concepts


# Code Companion

Each chapter includes practical Rust implementations:

👉 https://github.com/VinEckSie/sealed-in-rust/tree/main/rust_crypto_book_code/examples

Examples include:

- XOR and Feistel constructions
- AES vs ChaCha20 exploration
- RSA and Ed25519 implementations
- property-based tested modules


# Engineering Focus

- safe-by-design APIs
- property-based testing
- minimal and auditable implementations
- modular crate structure
- cryptography applied to real software systems


# Built With

Rust  
mdBook  
criterion  
doctests  

# Status

Chapters are released progressively with emphasis on:

correctness  
clarity  
maintainability  


# License

MIT OR Apache-2.0
