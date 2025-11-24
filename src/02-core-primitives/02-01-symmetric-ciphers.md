## Symmetric Ciphers: XOR, AES, ChaCha20 & Beyond

> 🔐 **Used in:** VPNs, TLS (post-handshake), disk encryption, messaging apps  
> ✅ Still foundational in modern cryptography.




### What Are Symmetric Ciphers?

Symmetric ciphers use the **same key** for both encryption and decryption. Unlike public-key cryptography, they don’t offer key exchange—but they are **much faster**, making them ideal for bulk data encryption.

They are used everywhere: encrypted file systems, secure communications, and even inside protocols like TLS (after the handshake).




### XOR Cipher — Simplicity That Teaches
> ⚠️ Insecure. Demonstration-only (used in educational demos, malware obfuscation )


Watch it on my *Fearless in Rust* channel: [XOR Cipher in Rust - Step by Step](https://www.youtube.com/watch?v=wA-p_c19ZFw&t=326s)

<!--
>⚠️ Annotate if a Rust crate exists + maturity level
-->
We first explored XOR encryption in [Section 1.4: First Code — A Naive XOR Encryptor](../01-foundations/01-04-first-code.md), where we built a full working example from scratch.


XOR is the simplest symmetric cipher: each byte of the message is XORed with a repeating key.
Reversibility is built-in — XORing twice with the same key restores the original.

```rust
fn main() {
    let message = b"Hi, Rust!";
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
> **🟢 Conclusion**
> XOR encryption is reversible and stateless, which makes it simple and fast. But it lacks confusion and diffusion, so patterns in the input remain visible — offering no real resistance to cryptanalysis.




### Feistel Networks — Foundation of Classic Block Ciphers
> ⚠️ Cryptographically obsolete, but conceptually important (used in DES[^DES], 3DES[^3DES])
<!--
> ⚠️️ Annotate if a Rust crate exists + maturity level
-->
Feistel networks are a clever way to build reversible encryption using any basic function—even if that function itself can’t be reversed. That’s the key idea.

Each round applies a transformation to the data. Multiple rounds are chained to strengthen security.

Each round does the following:
1. Takes two halves: Left (L) and Right (R)
2. Computes a function f(R, key)
3. Updates the pair as:
```vbnet
L₂ = R₁
R₂ = L₁ ⊕ f(R₁, key)
```

To encrypt, let’s see it in Rust:
```rust
fn feistel_round(l: u8, r: u8, k: u8) -> (u8, u8) {
    let f = r ^ k;
    (r, l ^ f)
}

fn main() {
    let left1: u8 = 0b1010_1010;  // 170
    let right1: u8 = 0b0101_0101; // 85
    let key: u8 = 0b1111_0000;   // 240

    let (left2, right2) = feistel_round(left1, right1, key);
    println!("Encrypted: ({}, {})", left2, right2);
}
```

Decryption reuses the same function f, simply reversing the round transformation:

```rust
fn feistel_round(l1: u8, r1: u8, k: u8) -> (u8, u8) {
    let f = r1 ^ k;
    (r1, l1 ^ f)
}

fn feistel_decrypt(l2: u8, r2: u8, k: u8) -> (u8, u8) {
    let f = l2 ^ k;
    let l1 = r2 ^ f;
    (l1, l2)
}

fn main() {
    let left1: u8 = 0b1010_1010;  // 170
    let right1: u8 = 0b0101_0101; // 85
    let key: u8 = 0b1111_0000;   // 240

    let (left2, right2) = feistel_round(left1, right1, key);
    println!("Encrypted: ({}, {})", left2, right2);

    let (left_orig, right_orig) = feistel_decrypt(left2, right2, key);
    println!("Decrypted: ({}, {})", left_orig, right_orig);
}
```
<br>
Because encryption produces :

```pgsql
Encrypted → (R, L ⊕ f(R, k))
```

Let’s define:
- L₁ and R₁ = original input
- L₂ = R₁ and R₂ = L₁ ⊕ f(R₁, k)


We receive (L₂, R₂) and want to recover (L₁, R₁):
1. From encryption, we know L₂ = R₁
   - So: R₁ = L₂

2. And: R₂ = L₁ ⊕ f(R₁, k)
   - Replace R₁ with L₂
   - R₂ = L₁ ⊕ f(L₂, k)

3. Rearranging to get L₁:
   - L₁ = R₂ ⊕ f(L₂, k)

<br>
So, decryption is

```pgsql
L₁ = R₂ ⊕ f(L₂, k)
R₁ = L₂
```

> **🟢 Conclusion**  
> Reversibility comes from XOR being reversible and swapping the halves.
Feistel networks let you build reversible encryption even with non-invertible functions.
This idea shaped DES and similar ciphers. 
> 
> Not used today due to known vulnerabilities, but conceptually essential.




### Substitution–Permutation Networks (SPN)
> ⚠️ Software-only S-box implementations can leak secrets through cache timing. Modern AES implementations use hardware instructions (AES-NI) or constant-time software libraries.

>  ⚠️ Used in AES[^AES], Camellia[^Camellia], and modern block ciphers.
> Still dominant in current cipher architectures

<!--
> ⚠️ Annotate if a Rust crate exists + maturity level
-->
Substitution-Permutation Networks (SPNs) are a powerful way to build secure block ciphers by layering simple operations repeated across multiple rounds to build a secure cipher. 

Each round does the following:
1. Substitution – replace each byte using an S-box (non-linear mapping)
2. Permutation – reorder bits or bytes to spread influence
3. Key mixing – XOR the block with a round key

Decryption reverses these steps in reverse order.


> 💡 An S-box (substitution box) is a predefined table that maps each input byte to a new output byte. 
> Its goal is to introduce non-linearity — meaning the output doesn't follow any simple, predictable rule based on the input. 
> <br><br>
> This non-linear mapping ensures that small changes in the input produce unpredictable changes in the output, making it impossible to reverse or model with linear equations — a key requirement for secure encryption.

<br>
Let’s walk through a simple encryption of a 4-byte block.

```rust
use std::convert::TryInto;

// Manually defined "shuffled" S-box (shortened for demo)
let s_box: [u8; 16] = [
   0x63, 0x7C, 0x77, 0x7B,
   0xF2, 0x6B, 0x6F, 0xC5,
   0x30, 0x01, 0x67, 0x2B,
   0xFE, 0xD7, 0xAB, 0x76,
];

// Step 1: Substitution with S-box
// ⚠️ input and output (substituted) must have the same size
// Otherwise, map() or indexing will panic at runtime
let input: [u8; 4] = [0x00, 0x03, 0x07, 0x0F];
let substituted: [u8; 4] = input.map(|b| s_box[b as usize]);

// Step 2: Permutation (custom byte reordering)
let permuted: [u8; 4] = [
   substituted[2], // byte 2 moves to pos 0
   substituted[0], // byte 0 → pos 1
   substituted[3], // byte 3 → pos 2
   substituted[1], // byte 1 → pos 3
];

// Step 3: XOR with round key
let round_key: [u8; 4] = [0xF0, 0x0F, 0xAA, 0x55];
let encrypted: [u8; 4] = permuted
   .iter()
   .zip(round_key.iter())
   .map(|(a, b)| a ^ b)
   .collect::<Vec<u8>>()
   .try_into()
   .unwrap();


println!("Step        | Byte 0 | Byte 1 | Byte 2 | Byte 3");
println!("------------|--------|--------|--------|--------");
println!("Input       | {:02X}     | {:02X}     | {:02X}     | {:02X}", input[0], input[1], input[2], input[3]);
println!("Substituted | {:02X}     | {:02X}     | {:02X}     | {:02X}", substituted[0], substituted[1], substituted[2], substituted[3]);
println!("Permuted    | {:02X}     | {:02X}     | {:02X}     | {:02X}", permuted[0], permuted[1], permuted[2], permuted[3]);
println!("Encrypted   | {:02X}     | {:02X}     | {:02X}     | {:02X}", encrypted[0], encrypted[1], encrypted[2], encrypted[3]);
```
<br>
To decrypt, reverse the steps in reverse order:

```rust
use std::convert::TryInto;

// Same S-box used for encryption
let s_box: [u8; 16] = [
   0x63, 0x7C, 0x77, 0x7B,
   0xF2, 0x6B, 0x6F, 0xC5,
   0x30, 0x01, 0x67, 0x2B,
   0xFE, 0xD7, 0xAB, 0x76,
];

// Generate inverse S-box
let mut inverse_s_box = [0u8; 256];
for (i, &val) in s_box.iter().enumerate() {
   inverse_s_box[val as usize] = i as u8;
}

// Encrypted block from the previous encryption output
let encrypted: [u8; 4] = [0x35, 0x6C, 0xDC, 0x2E];
let round_key: [u8; 4] = [0xF0, 0x0F, 0xAA, 0x55];

// Step 1: Undo XOR with round key
let xor_reversed: [u8; 4] = encrypted
        .iter()
        .zip(round_key.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

// Step 2:  Reverse permutation
// Remember: original permutation was [2, 0, 3, 1]
// So now we must do: [1, 3, 0, 2]
let permuted_reversed: [u8; 4] = [
   xor_reversed[1], // was originally at index 0
   xor_reversed[3], // was at index 1
   xor_reversed[0], // was at index 2
   xor_reversed[2], // was at index 3
];

// Step 3: Inverse substitution using inverse_s_box
let decrypted: [u8; 4] = permuted_reversed.map(|b| inverse_s_box[b as usize]);


println!("Step        | Byte 0 | Byte 1 | Byte 2 | Byte 3");
println!("------------|--------|--------|--------|--------");
println!("Encrypted   | {:02X}     | {:02X}     | {:02X}     | {:02X}", encrypted[0], encrypted[1], encrypted[2], encrypted[3]);
println!("XOR Rev     | {:02X}     | {:02X}     | {:02X}     | {:02X}", xor_reversed[0], xor_reversed[1], xor_reversed[2], xor_reversed[3]);
println!("Perm Rev    | {:02X}     | {:02X}     | {:02X}     | {:02X}", permuted_reversed[0], permuted_reversed[1], permuted_reversed[2], permuted_reversed[3]);
println!("Decrypted   | {:02X}     | {:02X}     | {:02X}     | {:02X}", decrypted[0], decrypted[1], decrypted[2], decrypted[3]);
```

Why it works

- Substitution = confusion → Hide relationships between plaintext and ciphertext
- Permutation = diffusion → Spread input influence across the block

These are Shannon’s two pillars of secure ciphers.

> 💡 Claude Shannon, widely considered the father of modern cryptography, introduced the concepts of confusion and diffusion in 1949 as the foundation of secure cipher design.

> **🟢 Conclusion**  
> Substitution-Permutation Networks provide a simple yet powerful structure for building symmetric ciphers. They deliver the critical properties of confusion and diffusion, as first formalized by Claude Shannon in his foundational work on cryptographic security.




### AES (Advanced Encryption Standard)  — The Global Symmetric Standard
> 💡 Used in TLS[^TLS], LUKS[^LUKS], SSH[^SSH], mobile apps, and FIPS-certified systems[^FIPS].  
> Secure, fast, and hardware-accelerated

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crates used: [aes](https://crates.io/crates/aes), [block_modes](https://github.com/RustCrypto/block-modes)

AES is a symmetric-key block cipher developed by Belgian cryptographers Vincent Rijmen and Joan Daemen. It was selected by NIST in 2001 as the successor to DES and 3DES.

AES operates on 128-bit blocks and supports key sizes of 128, 192, or 256 bits. It is based on a Substitution–Permutation Network (SPN) and runs 10, 12, or 14 rounds depending on the key length.

It is standardized by FIPS-197, ISO/IEC[^ISOIEC], and widely adopted in security protocols such as TLS, SSH, and IPsec[^IPSec]. AES is available in hardware on most modern CPUs, making it both fast and energy-efficient.

<br>

🧪 **Code Example: AES-128-CBC Encryption & Decryption in Rust** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))  
We’ll use the aes and block-modes crates to encrypt and decrypt a message using AES-128 in CBC mode[^CBC] with PKCS7[^PKCS7] padding.

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:aes}}
```

Output:
```rust,no_run
Ciphertext (hex): a2ae0699ff0bc71e6ff43a32531d88
Decrypted text: Attack at dawn!
```
✅ Use a unique IV (Initialization Vector) for every encryption, and never reuse a key/IV pair. Avoid ECB mode entirely, and prefer AEAD modes (e.g., AES-GCM) when available.

> **🟢 Conclusion**
>
> AES is the modern standard for symmetric encryption. 
>
> It is fast, secure, and hardware-accelerated — making it ideal for both embedded systems and high-throughput servers. 
>
> When used correctly with a secure mode like CBC or GCM and proper key/IV management, AES provides strong resistance against all known practical attacks.





### ChaCha20 — Modern Stream Cipher  

> 💡 A stream cipher encrypts data one bit or byte at a time by XORing it with a pseudorandom keystream, instead of encrypting fixed-size blocks like a block cipher.

> 💡 Used in WireGuard[^WIREGUARD], TLS (on non-AES hardware[^NONAES]), mobile apps, messaging protocols, and security libraries.  
> Fast, simple, and naturally resistant to timing attacks.

> <img src="../images/cargo.png" alt="My Crate Logo" width="22" style="vertical-align: middle; margin-right: 6px;"> Crate used: [chacha20](https://crates.io/crates/chacha20)


ChaCha20 is a modern stream cipher designed by Daniel J. Bernstein.  

It is the streamlined successor to Salsa20[^SALSA20], offering improved diffusion and exceptional performance on all platforms — especially devices without AES hardware.

Unlike block ciphers, ChaCha20 does not process data in fixed-size blocks. Instead, it transforms a key + nonce + counter into a pseudorandom keystream[^PSEUDOKEY]. 

Encryption is simply: ciphertext = plaintext XOR keystream

No padding. No block alignment. Just pure stream encryption.

ChaCha20 is now a fundamental primitive across modern cryptography:  
WireGuard, OpenSSH (for session keys), TLS 1.3 fallback ciphers, mobile operating systems, and many authenticated encryption schemes like ChaCha20-Poly1305[^POLY1305].

🧪 **Code Example: ChaCha20 Encryption** ([source code](https://github.com/VinEckSie/sealed-in-rust/blob/main/rust_crypto_book_code/src/lib.rs))

We’ll generate a ChaCha20 keystream and XOR it with a plaintext message.  
The API is extremely simple — you create a cipher and stream through it.

```rust,no_run
{{#include ../../rust_crypto_book_code/src/lib.rs:chacha20}}
```

Output:
```rust,no_run
Ciphertext (hex): b0bf118af914c7127eb12a3a4a1489c262dcdd53e9563bfaf652
Decrypted text: Secret meeting at midnight
```

> 🚨 **Security rule** 🚨
>
> **Never reuse the same (key, nonce) pair. Doing so reveals keystream reuse → instant compromise.**
> **ChaCha20 itself is secure, but it becomes unsafe if you repeat a nonce.**

> **🟢 Conclusion**
>
>ChaCha20 is the modern workhorse of stream ciphers: fast, extremely simple to implement correctly, and designed to avoid timing leaks by construction. 
>
> It performs brilliantly on laptops, phones, microcontrollers, and any platform lacking AES acceleration. 
>
> Use a fresh nonce for every encryption, treat keystreams as one-time pads, and avoid reuse. When authenticated encryption is needed, pair ChaCha20 with Poly1305 (ChaCha20-Poly1305).
>
>ChaCha20 combines security, clarity, and performance — a perfect fit for modern Rust systems, network protocols, and embedded environments.

### Modes of Operation

Modern cryptography doesn’t encrypt “messages.” It encrypts *blocks*.  
AES, for example, works on 128-bit chunks and nothing else.  
So to handle real-world data—files, network packets, logs, telemetry—we need a strategy to link those fixed-size blocks together safely.

That strategy is a mode of operation.

Modes are not decorative. They don’t sit on top of AES; they *define* its security. Pick the wrong one and your encryption leaks patterns. Pick the right one and you get confidentiality at scale.


**Why Modes Exist**

A block cipher is deterministic: same key + same block = same output.  
That predictability is deadly when encrypting long messages.

Modes solve four problems:

1. Randomness — injecting unpredictability so repeated blocks don’t look the same.  
2. Chaining — connecting blocks so tampering affects more than one piece.  
3. Streaming — letting you encrypt arbitrary sizes efficiently.  
4. State — defining how to start, continue, and finish encryption safely.

**Overview of Common Modes**

| Mode | Secure? | Real Use Case | Notes |
|------|---------|----------------|-------|
| **ECB** | ❌ No | None (except teaching) | Leaks structure. Never use in production. |
| **CBC** | ⚠️Risky | Legacy protocols | Requires a random IV. Padding mistakes break it. |
| **CTR** | ✅ Yes | High-speed streaming, networking, I/O | Turns AES into a fast stream cipher. Very robust when nonce-unique. |
| **XTS** | ✅ Yes | Disk and sector encryption | Designed for storage only, not general messages. |

**ECB — The Anti-Example**

ECB encrypts each block independently.  
Patterns in the plaintext appear in the ciphertext.  
Famous example: encrypting the Tux penguin still looks like a penguin.

If you see ECB in a system, assume the designer didn’t understand cryptography.

**CBC — The Old Workhorse**

CBC chains each block with the previous one using XOR.  
If the IV is truly random and padding is handled correctly, it’s fine.  
But historically, padding-oracle attacks destroyed its safety in many protocols.

Today, CBC mostly survives in legacy stacks and old file formats.  
New designs avoid it.

**CTR — The Modern Default**

CTR mode transforms AES into a stream cipher.  
Instead of encrypting the plaintext blocks directly, it encrypts a counter and XORs the result with the message.

This gives:

- high performance  
- random access  
- no padding  
- clean parallelism  

> 🚨
>The only hard rule: never reuse the same (key, nonce) pair.  
>Break that rule and attackers recover the XOR of two plaintexts.

**CTR Example**

```rust
use aes::Aes128;
use ctr::cipher::{KeyIvInit, StreamCipher};

let mut cipher = ctr::Ctr128BE::<Aes128>::new(key.into(), nonce.into());
cipher.apply_keystream(&mut buffer); // encrypt or decrypt
```

CTR is simple, fast, and well-suited to network protocols, telemetry pipelines, and embedded systems.

**XTS — Built for Storage**

XTS is AES-CTR + a “tweak” system tailored to disk sectors.
It prevents block relocation attacks and keeps each sector isolated.

XTS shines for full-disk encryption because it resists sector-copy tampering and is purpose-built for storage. It isn’t suitable for general messages or network data, and it requires two independent AES keys.

Use XTS only when you’re encrypting storage blocks.

**Key Takeaway**

Modes of operation are not optional add-ons.
They decide whether your encryption is safe or broken.

- ECB teaches you what not to do.

- CBC reminds you legacy systems carry hidden risks.

- CTR gives you modern, scalable encryption for streaming workloads.

- XTS protects disks—nothing else.

</br>

If you understand modes, you understand how real-world encryption actually works.


[^DES]: DES — early symmetric cipher (56-bit), now insecure. [More](../99-appendices/99-01-glossary.md#des-data-encryption-standard)  
[^3DES]: 3DES — DES applied three times, better than DES but now deprecated. [More](../99-appendices/99-01-glossary.md#3des-triple-des)  
[^AES]: AES — The modern global standard, fast, secure, and hardware-accelerated. [More](../99-appendices/99-01-glossary.md#aes-advanced-encryption-standard)
[^Camellia]: Camellia — Japanese block cipher, secure & AES-comparable. [More](../99-appendices/99-01-glossary.md#camellia)
[^TLS]: TLS — protocol securing data in transit (HTTPS, etc.). [More](../99-appendices/99-01-glossary.md#tls-transport-layer-security)  
[^LUKS]: LUKS — Linux standard for full disk encryption. [More](../99-appendices/99-01-glossary.md#luks-linux-unified-key-setup)  
[^SSH]: SSH — secure remote access protocol. [More](../99-appendices/99-01-glossary.md#ssh-secure-shell)  
[^FIPS]: FIPS — U.S. cryptographic standards for government/finance. [More](../99-appendices/99-01-glossary.md#fips-federal-information-processing-standards)  
[^ISOIEC]: ISO/IEC — international standards for IT/crypto. [More](../99-appendices/99-01-glossary.md#isoiec)  
[^IPSec]: IPSec — protocol suite for securing IP communications. [More](../99-appendices/99-01-glossary.md#ipsec)  
[^CBC]: CBC — block cipher mode, chains blocks for security. [More](../99-appendices/99-01-glossary.md#cbc-cipher-block-chaining)  
[^PKCS7]: PKCS7 — padding scheme for block ciphers. [More](../99-appendices/99-01-glossary.md#pkcs7)
[^WIREGUARD]: WireGuard — modern VPN protocol using ChaCha20-Poly1305 to secure IP traffic. [More](../99-appendices/99-01-glossary.md#wireguard)
[^NONAES]: Non-AES hardware — CPUs without AES instructions, where ChaCha20 is often faster than AES. [More](../99-appendices/99-01-glossary.md#non-aes-hardware)
[^SALSA20]: Salsa20 — stream cipher by Daniel J. Bernstein; predecessor of ChaCha20, fast and well-studied. [More](../99-appendices/99-01-glossary.md#salsa20)
[^PSEUDOKEY]: A pseudorandom keystream is a sequence of bits/bytes that looks random but is deterministically generated from a secret key (and usually a nonce). [More](../99-appendices/99-01-glossary.md#pseudorandom-keystream)
[^POLY1305]: ChaCha20-Poly1305 is an AEAD scheme that combines the ChaCha20 stream cipher with the Poly1305 MAC to provide authenticated encryption (confidentiality + integrity). [More](../99-appendices/99-01-glossary.md#chacha20-poly1305)
