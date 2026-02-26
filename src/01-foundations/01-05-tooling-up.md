## Tooling Up

Before we dive into cryptographic primitives, here’s a quick look at tools that can support your Rust development journey especially if you plan to write your own crypto libraries.

> None of these tools are required to read or complete the following chapters.
> If you already have `cargo` installed, you're good to go!

That said, for readers who want to build clean, secure, and testable codebases, these tools are worth bookmarking:

### Code Quality
- **[rustfmt](https://github.com/rust-lang/rustfmt)**: auto-formats code to keep it idiomatic
- **[clippy](https://github.com/rust-lang/rust-clippy)**: catches common pitfalls and non-idiomatic patterns

### Testing & Fuzzing
- **[proptest](https://docs.rs/proptest)**: property-based testing for edge case discovery
- **[cargo-fuzz](https://rust-fuzz.github.io/book/)**: fuzz testing to uncover panics and vulnerabilities

### Security & Auditing
- **[cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)**: alerts you to vulnerable crates
- **[cargo-geiger](https://github.com/rust-secure-code/cargo-geiger)**: scans for `unsafe` code

### Benchmarking & Debugging
- **[Criterion.rs](https://bheisler.github.io/criterion.rs/book/)**: precise performance benchmarks
- **[cargo-expand](https://github.com/dtolnay/cargo-expand)**: view macro-expanded code (useful when using `#[derive(...)]`)

As you gain experience, integrating these tools will help ensure your cryptographic code is not only correct but robust, maintainable, and audit-friendly.

### Try It Yourself

Want to skip the setup and jump right into coding?

👉 Use the [Sealed in Rust Starter Template](https://github.com/vinecksie/sealed-starter), a minimal Rust project preconfigured with the tools mentioned above.

```bash
git clone https://github.com/vinecksie/sealed-starter.git
cd sealed-starter
cargo test
```

<!--This project will evolve with the book and include examples, tests, and benchmarking code as you progress.-->
