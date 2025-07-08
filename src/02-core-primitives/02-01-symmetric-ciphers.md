## Symmetric Ciphers: XOR, AES, ChaCha20 & Beyond

> 🔐 **Used in:** VPNs, TLS (post-handshake), disk encryption, messaging apps  
> ✅ Still foundational in modern cryptography.

### What Are Symmetric Ciphers?

Symmetric ciphers use the **same key** for both encryption and decryption. Unlike public-key cryptography, they don’t offer key exchange—but they are **much faster**, making them ideal for bulk data encryption.

They are used everywhere: encrypted file systems, secure communications, and even inside protocols like TLS (after the handshake).


### XOR Cipher — Simplicity That Teaches
> ⚠️ Insecure. Demonstration-only (used in educational demos, malware obfuscation )

XOR is the simplest symmetric cipher: each byte of the message is XORed with a repeating key.
Reversibility is built-in — XORing twice with the same key restores the original.

```rust
fn main() {
    let message = b"hello world";
    let key = b"key";

    let encrypted = xor_encrypt(message, key);
    let decrypted = xor_encrypt(&encrypted, key);

    println!("Encrypted: {:x?}", encrypted);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
}

pub fn xor_encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}
```

The symmetric encryption can be reversible without storing state — but XOR lacks confusion, diffusion, and resists no attacks.

### Feistel Networks — Foundation of Classic Block Ciphers
> ⚠️ Cryptographically obsolete, but conceptually important (used in DES, 3DES)

Feistel networks are a clever way to build reversible encryption using any basic function—even if that function itself can’t be reversed. That’s the key idea.

Each round:
Takes two halves: Left (L) and Right (R)
Computes a function f(R, key)
Updates the pair as:
```vbnet
L₂ = R
R₂ = L ⊕ f(R, key)
```

To encrypt, let’s see it in Rust:
```rust
fn feistel_round(l: u8, r: u8, k: u8) -> (u8, u8) {
    let f = r ^ k;
    (r, l ^ f)
}

fn main() {
    let left: u8 = 0b1010_1010;  // 170
    let right: u8 = 0b0101_0101; // 85
    let key: u8 = 0b1111_0000;   // 240

    let (new_left, new_right) = feistel_round(left, right, key);
    println!("Encrypted: ({}, {})", new_left, new_right);
}
```

To decrypt, use the same function f, but rearranged:

```rust
fn feistel_round(l: u8, r: u8, k: u8) -> (u8, u8) {
    let f = r ^ k;
    (r, l ^ f)
}

fn feistel_decrypt(nl: u8, nr: u8, k: u8) -> (u8, u8) {
    let f = nl ^ k;
    let l = nr ^ f;
    (l, nl)
}

fn main() {
    let left: u8 = 0b1010_1010;  // 170
    let right: u8 = 0b0101_0101; // 85
    let key: u8 = 0b1111_0000;   // 240

    let (new_left, new_right) = feistel_round(left, right, key);
    println!("Encrypted: ({}, {})", new_left, new_right);

    let (left_orig, right_orig) = feistel_decrypt(new_left, new_right, key);
    println!("Decrypted: ({}, {})", left_orig, right_orig);
}
```

Because we know:
```pgsql
Encrypted → (R, L ⊕ f(R, k))
Decryption → L = NewRight ⊕ f(NewLeft, k)
```

<br>

Reversibility comes from XOR being reversible and Swapping the halves.

Feistel networks let you build reversible encryption even with non-invertible functions.
This idea shaped DES and similar ciphers. 
Not used today due to known vulnerabilities, but conceptually essential.

### Substitution–Permutation Networks (SPN)
🧠 Used in AES, Camellia, and modern block ciphers
✅ Still dominant in current cipher architectures

SPNs apply:

Substitution: introduce non-linearity via S-boxes

Permutation: shuffle bits or bytes to spread influence

Example (simplified S-box substitution):

```rust
let input: u8 = 0x53;
let s_box = [/* 256-byte S-box values */];
let output = s_box[input as usize];
```

✅ SPNs offer confusion and diffusion, the two critical pillars for strong symmetric ciphers, as defined by Shannon.

### AES — The Global Symmetric Standard
🧠 Used in TLS, LUKS, SSH, mobile apps, and FIPS-certified systems
✅ Secure, fast, and hardware-accelerated

AES (Advanced Encryption Standard) is a block cipher:

Block size: 128 bits

Key sizes: 128, 192, or 256 bits

Based on SPN design with 10–14 rounds

```rust
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
let ciphertext = cipher.encrypt_vec(plaintext);
```

✅ Strong against known attacks when used with a secure mode (not ECB), unique IVs, and good key hygiene.

### ChaCha20 — Modern Stream Cipher
🧠 Used in WireGuard, mobile apps, TLS (non-AES platforms)
✅ Fast, simple, and side-channel resistant

ChaCha20 is a stream cipher:

Converts key + nonce into a pseudorandom keystream

XORs that keystream with plaintext

No padding or block alignment required

```rust
use chacha20::cipher::{KeyIvInit, StreamCipher};

let mut cipher = chacha20::ChaCha20::new(key.into(), nonce.into());
let mut buffer = plaintext.to_vec();
cipher.apply_keystream(&mut buffer);
```

✅ Safe by default, especially against timing attacks.
🚨 Like all stream ciphers, never reuse a (key, nonce) pair.

### Modes of Operation
🧠 Used to extend block ciphers to arbitrary-length messages
✅ Secure when correctly applied; required for real-world encryption

Block ciphers like AES only process fixed-size blocks. Modes of operation define how to apply them to arbitrary-length messages.

Mode	Secure?	Use Case	Notes
ECB	❌ No	Educational only	Reveals structure (e.g. "Tux")
CBC	⚠️ Risky	Legacy systems	Needs random IV, padding errors
CTR	✅ Yes	Streaming / fast I/O	Turns AES into stream cipher
XTS	✅ Yes	Disk encryption	Sector-specific, not general use

CTR example:

```rust
use ctr::cipher::{KeyIvInit, StreamCipher};
let mut cipher = ctr::Ctr128BE::<Aes128>::new(key.into(), nonce.into());
cipher.apply_keystream(&mut buffer);
```

✅ Modes are not optional—they define the cipher’s real-world safety.

### AES vs. ChaCha20 — Which to Choose?
🧠 Used in virtually all secure communications
✅ Choice depends on platform, speed, and implementation safety

Feature	AES (with mode)	ChaCha20
Type	Block cipher	Stream cipher
Speed	Hardware-accelerated	Fast on all CPUs
Complexity	Higher	Lower
Side-channel	Depends on impl	Timing-safe by design
Use Cases	File encryption, TLS	VPNs, mobile, embedded

✅ Both are safe when used correctly. ChaCha20 is often the default when simplicity or portability is needed.

### A Glimpse at Key Derivation
🧠 Used in password-based encryption and key expansion
✅ Absolutely required in practice. Included here for context.

Users don’t enter 256-bit keys—they type passwords. These must be turned into cryptographic keys with KDFs like:

```rust
use hkdf::Hkdf;
use sha2::Sha256;

let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
let mut okm = [0u8; 32];
hk.expand(b"info", &mut okm).unwrap();
```

✅ You'll revisit this in depth later. For now, remember: never use raw passwords as encryption keys.

### Summary
🧠 Symmetric ciphers are the engine room of practical cryptography
✅ This chapter builds your foundation for all applied encryption

You now understand:

Why symmetric encryption matters

How XOR, Feistel, and SPNs evolved into AES

When and how to use AES or ChaCha20

Why modes are critical—and where they fail

That encryption alone isn’t enough (spoiler: integrity comes next)

``yaml

---

Ready to paste into your book repo. Let me know when you're ready to move on to the **next chapter: MACs and AEAD**, or if you want me to generate `.md` files for past/future chapters too.
```
