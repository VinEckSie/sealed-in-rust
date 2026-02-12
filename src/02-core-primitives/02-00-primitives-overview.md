## How Ciphers, Hashes, MACs, AEAD Compose
This book introduces several cryptographic primitives.
They are not isolated tools. They are building blocks that compose into secure systems.

This section shows how they connect.

Each following chapter will explore one of these components in depth.

## The Four Core Primitives

### Symmetric Cipher
Provides confidentiality.
Encrypts data using a shared secret key.
### Hash Function
Provides integrity fingerprinting.
Maps arbitrary data to a fixed-size digest. No key.
### MAC (Message Authentication Code)
Provides integrity + authenticity.
Built using a secret key (often from a hash construction such as HMAC).
### AEAD (Authenticated Encryption with Associated Data)
Provides confidentiality + integrity + authenticity in a single construction.

## Composition Hierarchy

Hash
→ Used to build MAC (HMAC)

Cipher + Authenticator
→ Used to build AEAD

AEAD
→ Used in modern secure communication protocols

## Mental Model

| Goal | Primitive |
|------|---------|
| Hide data | Symmetric cipher |
| Detect Modification | Hash |
| Prove authenticity | MAC |
| Hide + authenticate | AEAD |


> **🚨 Security rule 🚨**
>
> **Never invent your own composition.**
>
> ❌ Bad idea: encrypt(data) + hash(data)
>
> ✅ Correct approach: Use AEAD.
