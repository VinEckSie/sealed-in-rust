## Cryptographic Hashes: SHA-2, BLAKE3 & beyond

> 🔐 **Used in:** password storage, digital signatures, blockchains, TLS, integrity checks
>
> ✅ One-way, deterministic, and foundational to *all* modern cryptography


### What Is a Cryptographic Hash?
A cryptographic hash function takes arbitrary input data and produces a fixed-size output, called a *digest*.

Key properties:

- Deterministic:  same input → same output
- One-way:  infeasible to recover the input
- Collision-resistant:  infeasible to find two inputs with the same hash
- Avalanche effect:  tiny input change → completely different output

**Hash functions are not encryption.There is no key. There is no decryption.**

They exist to answer one question: *“Has this data changed and can I trust it?”*

### What Hashes Are Used For (Real Systems)
- Password storage (never store plaintext passwords)
- Digital signatures (hash → sign)
- TLS certificates (hashing messages before authentication)
- Package managers (verify downloads)
- Git (content-addressed storage)
- Blockchains (immutability and chaining)

**If symmetric ciphers protect confidentiality, hashes protect integrity and identity.**


### One-Way by Design

Consider this:

```text
"hello"  →  2cf24dba5fb0a30e26e83b2ac5b9e29e...
"hellO"  →  0d4a1185eecb25c46b7a5d3bca4f4f8b...

One flipped bit >> Completely different output
```

This is not accidental, it’s the core security property.

### SHA-2: the conservative tandard
> 💡 Used in TLS, certificates, package signing, blockchains. Stable, conservative, and widely trusted

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [sha2](https://crates.io/crates/sha2)

SHA-2[^sha2] is a family of hash functions standardized by NIST[^nist]:
- SHA-224[^sha224]
- SHA-256[^sha256]
- SHA-384[^sha384]
- SHA-512[^sha512]

SHA-256 is the most common.
It is slow enough to be secure, fast enough for general use and extremely well analyzed.
SHA-2 is boring and that’s a compliment.

🧪 **Code Example: SHA-256 Hashing** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs#L121))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:sha256}}
```

Output:
```text
SHA-256: b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
```

Same input. Same output. Always.

> **🟢 Conclusion**
> SHA-2 is the safe default.
> If you don’t know which hash to use, SHA-256 will almost never be the wrong answer.

### Why Hashes Are Not Password Protection
If you do this, you are already vulnerable.
```rust,no_run
hash(password)
```
because Hashes are fast, attackers can try billions per second, rainbow tables exist.

We fix this with KDFs[^kdf] (Argon2[^argon2], scrypt[^scrypt]). See [Key Derivation section](../02-core-primitives/02-01-symmetric-ciphers.md#a-glimpse-at-key-derivation)

> **🚨 Fast hashes are dangerous for passwords.**

### BLAKE3: modern, fast, and arallel
>💡 Used in modern systems, content addressing, integrity pipelines. Extremely fast, secure, and designed for today’s hardware

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [blake3](https://crates.io/crates/blake3)


BLAKE3[^blake3] is the modern evolution of BLAKE2:
- Cryptographically secure
- Massively parallel
- Faster than SHA-2
- Designed for multicore CPUs and SIMD[^simd]
- Unlike SHA-2, BLAKE3 was designed after modern hardware existed.

It also supports:
- Streaming
- Incremental hashing[^inc]
- Keyed hashing[^key]
- Extendable output (XOF)[^ext]

🧪 **Code Example: BLAKE3 Hashing** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs#L133))

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:blake3}}
```

Output:
```text
BLAKE3: d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24
```

### SHA-2 vs. BLAKE3: which should you use?

| Property       | SHA-256              | BLAKE3             |
|----------------|----------------------|--------------------|
| Standardized   | ✅ Yes               | ❌ Not NIST         |
| Speed          | ⚠️ Moderate          | 🚀 Extremely fast   |
| Parallelism    | ❌ Limited           | ✅ Built-in         |
| Maturity       | 🧱 Very conservative | 🧠 Modern design    |
| Simplicity     | ✅ Simple            | ✅ Simple           |
| Use cases      | ✅ Compliance        | ✅ Performance      |
|                | ✅ Interoperability  | ✅ Content hashing  |
|                |                      | ✅ Modern systems   |


### Collision Resistance: what “Breaking a Hash” really means
Breaking a hash means one of two things:
- Finding a collision[^collision] faster than brute force[^bruteforce]
- Finding a preimage[^preimage] faster than brute force

For SHA-256:
- No practical attacks exist
- Collision cost ≈ 2¹²⁸
- Preimage cost ≈ 2²⁵⁶

That is astronomically infeasible.

> 💡 Most real-world failures come from misuse, not broken math.

### Hashes in Real Protocols
Hashes are rarely used alone.

They appear inside:
- HMAC: keyed hashing for authentication
- HKDF: key derivation
- Digital signatures: sign(hash(message))
- Merkle trees[^merkle]: integrity structures
- Password KDFs: slow hashing

> **🟢 Conclusion**
>
>Hash functions provide integrity, not secrecy.
>
>They are one-way by design:
>- SHA-2 is conservative and universal
>- BLAKE3 is modern, fast, and scalable
>Hashes alone are not password protection
>
> Cryptographic hashes are the glue of modern security.
>
> They verify data, anchor trust, protect identities, and power nearly every cryptographic construction you’ll encounter.
> If encryption hides secrets, hashes define truth.


[^sha2]: SHA-2 (Secure Hash Algorithm 2): NIST-standardized family of cryptographic hash functions including SHA-224, SHA-256, SHA-384, and SHA-512. [More](../99-appendices/99-01-glossary.md#sha-2-secure-hash-algorithm-2)
[^blake3]: BLAKE3 (named after the BLAKE hash family): Modern cryptographic hash optimized for speed, parallelism, and  implicity on modern hardware. [More](../99-appendices/99-01-glossary.md#blake3)
[^nist]: NIST (National Institute of Standards and Technology): U.S. authority responsible for standardizing widely used cryptographic algorithms. [More](../99-appendices/99-01-glossary.md#nist)
[^sha224]: SHA-224: SHA-2 variant producing a 224-bit digest, derived from SHA-256. [More](../99-appendices/99-01-glossary.md#sha-224)
[^sha256]: SHA-256: Most widely deployed SHA-2 hash function, offering strong collision and preimage resistance. [More](../99-appendices/99-01-glossary.md#sha-256)
[^sha384]: SHA-384: SHA-2 variant optimized for 64-bit systems with higher security margin. [More](../99-appendices/99-01-glossary.md#sha-384)
[^sha512]: SHA-512: SHA-2 variant with the largest output size and best performance on 64-bit architectures. [More](../99-appendices/99-01-glossary.md#sha-512)
[^kdf]: KDF (Key Derivation Function): Function that transforms a weak or strong secret into cryptographically secure keys for use with symmetric encryption. [More](../99-appendices/99-01-glossary.md#kdf)
[^argon2]: Argon2 (named after the mythological ship Argo and its crew, the Argonauts): The modern standard for password hashing and key derivation, designed to resist GPU and ASIC attacks using memory-hard computation. [More](../99-appendices/99-01-glossary.md#argon2)
[^scrypt]: Scrypt: A memory-hard password-based key derivation function built to make large-scale hardware brute-force attacks expensive and inefficient. Older than Argon2 but still widely used. [More](../99-appendices/99-01-glossary.md#scrypt)
[^simd]: SIMD (Single Instruction, Multiple Data): CPU execution model that applies the same operation to multiple data elements in parallel for high performance. [More](../99-appendices/99-01-glossary.md#simd)
[^inc]: Incremental hashing: Hashing method that processes input in chunks, allowing streaming and large data hashing without loading everything into memory. [More](../99-appendices/99-01-glossary.md#incremental-hashing)
[^key]: Keyed hashing: Hash function variant that incorporates a secret key to provide message authentication and integrity guarantees. [More](../99-appendices/99-01-glossary.md#keyed-hashing)
[^ext]: Extendable output (XOF): Hash construction that can generate an arbitrary-length output stream from a single input and key. [More](../99-appendices/99-01-glossary.md#extendable-output-function)
[^collision]: Collision: Existence of two distinct inputs that produce the same hash output, undermining a hash function’s uniqueness guarantee. [More](../99-appendices/99-01-glossary.md#collision)
[^bruteforce]: Brute force: Exhaustive attack that tries all possible inputs or keys until the correct one is found. [More](../99-appendices/99-01-glossary.md#brute-force)
[^preimage]: Preimage: Original input corresponding to a given hash output; preimage resistance means it is infeasible to recover this input. [More](../99-appendices/99-01-glossary.md#preimage)
[^merkle]: Merkle tree: Tree data structure where each node is a hash of its children, enabling efficient and secure data integrity verification. [More](../99-appendices/99-01-glossary.md#merkle-tree)
