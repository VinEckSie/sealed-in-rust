# How to Use This Book

This book is designed to be practical, modular, and domain-focused.  
You can read it front to back, or jump directly to the domains and primitives most relevant to your work.


## Structure

The book is organized into four major parts:

**Foundations** — Why Rust is uniquely suited for secure cryptographic engineering  
**Primitives** — The core building blocks of cryptography, with Rust-focused usage and implementation  
**Applied Domains** — Real-world systems and how they use cryptography in practice (blockchain, defense, aerospace, medical, infrastructure, etc.)  
**Crate Building** — How to architect, test, audit, and publish your own cryptographic crate in Rust

###
Each chapter includes:
- Plain-language explanations
- Rust code examples using community crates
- Security insights and common pitfalls
- Domain-specific applications

## Code & Examples

This book includes real, runnable Rust code for nearly every chapter.

- All examples are written for **Rust 2024**
- You’ll need a working Rust toolchain (`rustup`, `cargo`)
- Each chapter’s code is provided as a separate **Cargo example**.
- Examples live inside the companion repository: [sealed-in-rust](https://github.com/VinEckSie/sealed-in-rust/tree/main/rust_crypto_book_code)

You can run them locally without touching production crates or complex setups:

```bash
git clone https://github.com/VinEckSie/sealed-in-rust.git
cd sealed-in-rust/rust_crypto_book_code
cargo run --example aes_cbc //Replace aes_cbc with any chapter example you want to try
```

> ⚠️ **Note on Playground Limitations:**  
> Some examples in this book use no_run or ignore markers when rendered online, because external crates like aes, cbc, or rsa cannot be compiled in-browser.
> All examples work locally, and full source code is always included.

> 💡 These examples are didactic and minimal by design.
> They illustrate cryptographic concepts — not replace mature, production-ready libraries.

---

## Non-linear Reading

This book doesn’t assume linear progress.

- Want to build a secure file encryption tool? **Symmetric Ciphers** + **Secure Infrastructure**
- Curious about smart contracts? Go directly to **ECC** and **Blockchain & Web3**
- Working on embedded firmware? Check out **Ascon**, **PRESENT**, and **Defense & Aerospace**

Each domain chapter reminds you of the necessary primitives — like a map, not a locked path.


## Contributing, Feedback, & Issues

This book is a living project.

- Errors? Open an issue or PR on the [GitHub repo](https://github.com/VinEckSie/sealed-in-rust)
- Suggestions? You’re welcome to share ideas for domains, examples, or improvements
- Contributions to the examples repo are also welcome


## Final Note

This is not a cryptography textbook — it’s a cryptographic engineering manual.  
By the end, you’ll not only understand the primitives, you’ll know how to use them to secure real systems — in Rust, by design, not by accident.

Let’s begin.
