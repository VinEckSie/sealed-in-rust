## Tooling Up

Before we dive deeper into cryptographic primitives, let's set up our toolbox. In Rust, safety begins before you write code — it starts with choosing the right tools.

### Compiler and Toolchain
Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br>
Prefer cargo new --lib for cryptographic crates

Enable relevant features in Cargo.toml
Example:

```toml
[dependencies]
hex = "0.4"
rand = "0.8"
```

### Linting & Formatting
rustfmt — keep your code idiomatic

clippy — catch logic errors and subtle anti-patterns

Useful alias:

```bash
alias checkall="cargo fmt -- --check && cargo clippy -- -D warnings && cargo test"
```

### Testing & Fuzzing

proptest --- Generates randomized inputs to test your code against edge cases automatically.

cargo-fuzz --- Finds vulnerabilities by feeding random inputs and monitoring for panics or crashes.

Write tests early using #[cfg(test)]

Use property-based testing with proptest:

```toml
[dev-dependencies]
proptest = "1.4"
```

Fuzzing with cargo-fuzz

```bash
cargo install cargo-fuzz
cargo fuzz init
```

### Audit & Security Tools
cargo --- audit to catch vulnerable dependencies:

```bash
cargo install cargo-audit
cargo audit
```

Dependency pinning with Cargo.lock

Review unsafe blocks early and often

### Other Useful Tools
criterion.rs — for benchmarking crypto algorithms

cargo-expand --- view macro-expanded code

cargo-geiger --- scan for unsafe code:

```bash
cargo install cargo-geiger
cargo geiger
```

<br><br>
With the right tools in place, you're ready to build cryptographic systems that are not only secure — but readable, maintainable, and testable. Let’s move from theory to code.

