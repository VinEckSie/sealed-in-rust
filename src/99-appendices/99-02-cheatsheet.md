# Cryptographic Concepts Cheatsheet


*This cheatsheet is designed as a quick mental map. For detailed explanations and Rust code, see Part 2 and Part 3 of this book.*

---

## 🔐 Core Principles

| Concept              | Summary                                                                 |
|----------------------|-------------------------------------------------------------------------|
| **Kerckhoffs’ Principle** | A cryptosystem must be secure even if everything except the key is known |
| **Shannon’s Maxim**  | The enemy knows the system — don’t rely on obscurity                    |
| **Perfect Secrecy**  | Ciphertext reveals no information without the key (e.g., OTP)           |
| **Semantic Security**| An attacker can’t learn anything new from ciphertext                    |

---

## 🔢 Mathematical Foundations

| Concept                 | Summary                                                                 |
|--------------------------|-------------------------------------------------------------------------|
| **Entropy**             | A measure of randomness; critical for secure key generation              |
| **Modular Arithmetic**  | Math used in most crypto (e.g., *a mod n*)                              |
| **Finite Fields**       | Algebraic structures where crypto operations like ECC take place         |
| **Primes & Factorization** | Basis for RSA’s difficulty: factoring large numbers is hard             |
| **Discrete Log Problem**| Hardness assumption behind Diffie-Hellman and ECC                       |

---

## 🔄 Encryption Concepts

| Concept               | Summary                                                                 |
|-----------------------|-------------------------------------------------------------------------|
| **Symmetric Encryption** | Same key used for encryption and decryption (e.g., AES)                |
| **Asymmetric Encryption** | Public-key systems like RSA and ECC                                   |
| **Block Cipher**      | Encrypts fixed-size blocks (e.g., AES-128)                              |
| **Stream Cipher**     | Encrypts bit-by-bit or byte-by-byte (e.g., ChaCha20)                    |
| **Modes of Operation**| Techniques to apply block ciphers to arbitrary-length data (e.g., CBC, GCM) |
| **Padding**           | Fills the last block in block cipher (e.g., PKCS#7)                     |

---

## 🔁 Cryptographic Properties

| Concept                   | Summary                                                                |
|----------------------------|------------------------------------------------------------------------|
| **Confusion**             | Makes the relationship between key and ciphertext complex               |
| **Diffusion**             | Spreads the influence of each input bit across the ciphertext           |
| **Avalanche Effect**      | Small change in input → large change in output                         |
| **Deterministic Encryption** | Same ciphertext for same input+key — *not semantically secure*        |
| **Non-deterministic Encryption** | Includes randomness (like IVs) to ensure unique ciphertexts       |

---

## 🔐 Hash Functions

| Concept                   | Summary                                                                |
|----------------------------|------------------------------------------------------------------------|
| **Collision Resistance**  | It’s hard to find two different inputs with the same hash               |
| **Preimage Resistance**   | Given a hash, it’s hard to find an input that produces it              |
| **Second Preimage Resistance** | Given an input, hard to find another that hashes to the same value  |
| **Birthday Paradox**      | Hash collisions can happen surprisingly early (~2^n/2 complexity)       |
| **Merkle–Damgård Construction** | A method used in many hash functions like SHA-1, SHA-256          |

---

## 🔏 Digital Signatures

| Concept                   | Summary                                                                |
|----------------------------|------------------------------------------------------------------------|
| **Authenticity**          | Verifies that the message comes from the claimed sender                |
| **Non-repudiation**       | Signer cannot deny having signed                                       |
| **ECDSA / RSA Signatures**| Algorithms for digital signatures using asymmetric keys                |

---

## 📡 Protocol Concepts

| Concept                   | Summary                                                                |
|----------------------------|------------------------------------------------------------------------|
| **Key Exchange**          | Securely establishing a shared key over an insecure channel (e.g., Diffie-Hellman) |
| **Forward Secrecy**       | Compromise of one key doesn’t expose past sessions                     |
| **Replay Attack**         | Re-sending valid data to trick the system again                        |
| **Man-in-the-Middle**     | Attacker intercepts communication between two parties                  |
| **Nonce**                 | A number used once — prevents replay attacks and ensures uniqueness    |
| **Initialization Vector (IV)** | Random value to ensure unique ciphertexts in block cipher modes   |

---

## 🛡️ Attack Models

| Concept                   | Summary                                                                |
|----------------------------|------------------------------------------------------------------------|
| **Ciphertext-only attack** | Attacker only has access to encrypted messages                         |
| **Known-plaintext attack** | Attacker knows some plaintext–ciphertext pairs                         |
| **Chosen-plaintext attack**| Attacker can choose plaintexts and get their ciphertexts               |
| **Chosen-ciphertext attack**| Attacker can decrypt chosen ciphertexts                               |
| **Side-channel attack**    | Exploits physical leaks (timing, power, EM radiation)                  |

---
