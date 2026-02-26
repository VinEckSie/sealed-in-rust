## First Code: A Naive XOR Encryptor

Let’s write our first cryptographic algorithm or at least something that *looks* like one.

We’ll implement a simple **XOR cipher**. This method is *insecure* and should never be used in real applications, but it's the perfect teaching tool.


### What’s a Cipher?

A **cipher** is just a method to **transform readable data (plaintext)** into **unreadable data (ciphertext)** using a **key** and vice versa.

> 🧭 Word Origin: “Cipher”
The word comes from the Arabic "ṣifr" (صفر), meaning "zero" or "empty". It passed through Latin (cifra), then into French and English as cipher.<br><br>
> What started as a symbol for “nothing” evolved into a word for secret writing and eventually, encryption algorithms.

### What is XOR?

XOR stands for **"exclusive or"**, a bitwise operation:

| A | B | A XOR B |
|---|---|---------|
| 0 | 0 |   0     |
| 0 | 1 |   1     |
| 1 | 0 |   1     |
| 1 | 1 |   0     |


In short: XOR returns 1 if the bits differ, 0 if they’re the same.

The XOR operation flips bits when they differ:
```text
1 ^ 0 = 1
1 ^ 1 = 0
0 ^ 0 = 0
```

When used for encryption:
```text
cipher = plaintext ^ key
plaintext = cipher ^ key
```

That’s why XOR can be used to encrypt and decrypt data. If you XOR something twice with the same key, you get the original back.

> ✅ Simple, reversible, fast but also dangerously weak when misused.

### XOR, Bit by Bit
To truly understand XOR in cryptography, it helps to look at bit-level behavior.

Let’s say you compute:
```rust,noplayground
100 ^ 1
```

This doesn’t mean 100 to the power of 1. In Rust, ^ is the bitwise XOR operator.

Step-by-step:
```yaml
100 = 0110 0100
1   = 0000 0001
---------------
XOR = 0110 0101 = 101
```

✅ Each bit is compared:
If they’re different → 1
If they’re the same → 0

```rust,noplayground
100 ^ 1 = 101
```

This is what makes XOR useful: you can toggle bits with a key, and reverse it by applying the same key again.

### Why This?

This example teaches you:

- The reversible nature of XOR (`a ^ b ^ b == a`)
- Handling bytes and slices in Rust
- Thinking about encryption as a transformation
- Why key reuse and simplicity are dangerous

### Naive XOR in Rust

Here’s how to implement a basic XOR encryptor in Rust: <br>

Filename: src/main.rs
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

### Output

The output will show the encrypted bytes (in hex) and the original decrypted message.


### What’s Wrong With This Cipher?

- Key reuse makes patterns obvious
- No randomness or initialization vector (IV)
- Susceptible to frequency analysis attacks

<br>
This cipher is insecure but it demonstrates important cryptographic concepts:

- Reversibility
- Byte-wise transformations
- Why randomness and key handling matter

You’ll build on this when implementing real-world ciphers like ChaCha20 or AES.
