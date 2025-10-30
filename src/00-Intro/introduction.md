# Introduction

**Cryptography is everywhere — and yet most of us treat it like magic.**  

From encrypted chats and online banking to firmware updates on satellites, the systems we rely on daily are secured (or compromised) by cryptography. And too often, it fails — not because the math was wrong, but because the implementation was.

**This book exists to change that.**  

You’re not here to memorize equations. You’re here to understand, implement, and apply cryptographic primitives to real-world systems using a language designed to prevent mistakes before they happen: **Rust**.

Whether you're building infrastructure, smart contracts, embedded firmware, or secure APIs, this book gives you the tools to use cryptography safely, idiomatically, and fearlessly — one primitive, one project, one domain at a time.

## ⚠️ Important Note on Scope

This book takes a two-layered approach:

**Learning the Primitives**
- We begin with fundamental building blocks (XOR, AES, ChaCha20, Feistel, SPN) to teach how cryptography works under the hood.
- These examples are simplified, sometimes insecure by design, and are provided for educational clarity. 

**Applying Secure Constructions**
- For every primitive introduced, we also demonstrate how it is used safely in practice — with production-ready Rust code and well-reviewed crates.
- In modern systems, raw primitives are never used alone. Instead, we rely on AEAD (Authenticated Encryption with Associated Data) modes such as AES-GCM or ChaCha20-Poly1305, which provide both confidentiality and integrity/authentication.
- Each chapter connects the primitive to its real-world domains (e.g., AES-GCM in TLS, ChaCha20-Poly1305 in VPNs and mobile messaging, AES-XTS in disk encryption).

**💡 Bottom line:**
You’ll gain an understanding of the core mechanics of symmetric ciphers, and also learn how to apply them correctly with Rust in production scenarios. By the end, you’ll not only know what’s inside the black box, but also how to choose and use the right construction for your specific domain.

## Who This Book Is For

- Rust developers who want to understand and apply cryptography
- Security engineers transitioning to Rust
- Curious hackers tired of black-box crypto
- Systems developers who care about safety, correctness, and resilience

## What You’ll Learn

- The foundations and mental models behind symmetric and asymmetric cryptography
- How to use modern cryptographic crates in Rust safely and idiomatically
- Where cryptographic primitives show up in real-world domains (blockchain, embedded, medical, etc.)
- How to design, test, and publish your own secure Rust crypto crate

## What This Book Is Not

- ❌ A math-heavy cryptography textbook
- ❌ A copy-paste cookbook
- ❌ A blockchain hype manual

We won’t drown you in proofs, but we’ll explain just enough math to build intuition. We’ll write real code — not just use libraries. And we’ll focus on systems-level crypto, not speculative tokens.

## What You’ll Need

- ✅ Basic experience with Rust (enough to build a CLI or follow `cargo run`)
- ✅ Comfort with reading code, refactoring, and using crates
- ✅ Curiosity, and a bias toward safe, practical, applied learning

## Let’s begin
This book won’t make you a cryptographer in the academic sense — but it will make you something just as rare and valuable:
A Rust engineer who understands, wields, and applies cryptography with precision, context, and confidence.

You won’t just read — you’ll build.<br>
[Focused implementations and applied examples](https://github.com/VinEckSie/sealed-in-rust-book-code) are available here and updated weekly.