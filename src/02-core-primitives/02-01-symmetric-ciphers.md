## Symmetric Ciphers: XOR, AES, ChaCha20 & Beyond

> 🔐 **Used in:** VPNs, TLS (post-handshake), disk encryption, messaging apps  
> ✅ Still foundational in modern cryptography.


### What Are Symmetric Ciphers?

Symmetric ciphers use the **same key** for both encryption and decryption. Unlike public-key cryptography, they don’t offer key exchange—but they are **much faster**, making them ideal for bulk data encryption.

They are used everywhere: encrypted file systems, secure communications, and even inside protocols like TLS (after the handshake).


### XOR Cipher — Simplicity That Teaches
> ⚠️ Insecure. Demonstration-only (used in educational demos, malware obfuscation )
<!--
> > ![youtube](https://github.com/user-attachments/assets/e6eeacfc-ba92-4ecf-bb7b-48ad2384c1ae)   Watch it on my *Fearless in Rust* channel:  [XOR Cipher in Rust - Step by Step](https://www.youtube.com/watch?v=YOUR_VIDEO_ID)
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
> ⚠️ Cryptographically obsolete, but conceptually important (used in DES, 3DES)
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
>  ⚠️ Used in AES, Camellia, and modern block ciphers.
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

