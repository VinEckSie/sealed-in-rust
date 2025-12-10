# Glossary of Terms


## AES (Advanced Encryption Standard)
AES is a symmetric-key block cipher standardized by NIST in 2001, designed by Vincent Rijmen and Joan Daemen.
It operates on 128-bit blocks with keys of 128, 192, or 256 bits, and is based on a Substitution–Permutation Network.
AES is fast, secure, and widely implemented in both software and hardware (including CPU instructions).
It remains the global standard for symmetric encryption, used in TLS, SSH, disk encryption, and countless applications.

---
## ECB (Electronic Code Book)
ECB is the simplest block cipher mode of operation: it encrypts each block independently with the same key.
While easy to implement, it is insecure because identical plaintext blocks produce identical ciphertext blocks.
This leaks patterns in the input (e.g., images encrypted with ECB visibly preserve outlines).
Because of this, ECB is almost never used in practice and is considered unsafe.

---

## RSA (Rivest–Shamir–Adleman)
RSA is one of the first public-key cryptosystems, invented in 1977 by Ron Rivest, Adi Shamir, and Leonard Adleman.
It allows secure key exchange, encryption, and digital signatures by relying on the mathematical difficulty of factoring large integers.
RSA keys are typically 2048 or 3072 bits today. While still widely used, modern protocols increasingly migrate to elliptic-curve cryptography (ECC) for better performance and smaller key sizes.

---

## Padding in RSA
RSA encryption without padding is deterministic: encrypting the same message always produces the same ciphertext.
This makes it insecure, as attackers can guess messages or exploit mathematical properties of RSA.
Padding schemes (e.g., PKCS#1 v1.5 or OAEP) add randomness or structured bytes before encryption.
This ensures different ciphertexts for the same plaintext and provides protection against chosen-plaintext or chosen-ciphertext attacks.
Proper padding is essential for RSA’s security in practice.

---

## DES (Data Encryption Standard)
DES is a symmetric-key block cipher standardized in the 1970s by NIST.
It uses a 56-bit key and operates on 64-bit blocks.
Once a global standard, DES is now considered insecure due to its short key length, which makes it vulnerable to brute-force attacks.

---

## 3DES (Triple DES)
Triple DES (3DES) is an extension of DES that applies the DES algorithm three times with either two or three different keys.
This increases the effective key length to 112 or 168 bits.
It offered better security than DES but is now deprecated due to its slow performance and known cryptographic weaknesses.

---

## Camellia
Camellia is a modern symmetric-key block cipher developed in Japan by Mitsubishi Electric and NTT.
It offers security and performance comparable to AES, supporting key sizes of 128, 192, and 256 bits.
Camellia is standardized by ISO/IEC and the NESSIE project.
Though less widely used than AES, it is considered secure and suitable for both software and hardware implementations.

---

## TLS (Transport Layer Security)
TLS is a protocol defined by the IETF to secure data during transmission, most notably in HTTPS.
It uses asymmetric encryption for the handshake and key exchange, then switches to symmetric encryption for efficient bulk data encryption.
TLS provides confidentiality, integrity, and authentication for internet communications.

---

## LUKS (Linux Unified Key Setup)
LUKS is the standard format for full disk encryption on Linux, designed by Clemens Fruhwirth.
It provides strong symmetric encryption for protecting data at rest and supports multiple key slots, allowing different passphrases or keys to unlock the same volume.

---

## SSH (Secure Shell)
SSH is a secure remote access protocol invented by Tatu Ylönen in 1995.
It uses asymmetric encryption to authenticate users and symmetric encryption to secure the session once established.
SSH replaced insecure protocols such as Telnet and rlogin and is now the standard for secure remote administration.

---

## FIPS (Federal Information Processing Standards)
FIPS are cryptographic standards defined by NIST for use in U.S. government systems.
FIPS certification ensures that cryptographic algorithms and implementations meet strict security requirements.
It is often required in government, healthcare, and financial sectors to guarantee compliance and trust.

---

## ISO/IEC
ISO (International Organization for Standardization) and IEC (International Electrotechnical Commission) jointly publish international standards for information technology, including cryptography.
These standards ensure interoperability and security across implementations worldwide.

---

## IPSec
IPSec (Internet Protocol Security) is a suite of protocols designed to secure IP communications.
It operates at the network layer, providing encryption, integrity, and authentication for IP packets.
IPSec is widely used for VPNs and site-to-site secure tunnels.

---

## CBC (Cipher Block Chaining)
CBC is a block cipher mode of operation that chains blocks together for added security.
Each plaintext block is XORed with the previous ciphertext block before encryption, preventing identical plaintexts from producing identical ciphertexts.
An Initialization Vector (IV) is required for the first block to ensure uniqueness.
While stronger than ECB, CBC has weaknesses and should be used with caution.

---

## PKCS7
PKCS7 is a padding scheme used in block cipher encryption.
It fills up the last block of plaintext with bytes all set to the value of the number of padding bytes added.
For example, if 4 bytes of padding are needed, the block is filled with `04 04 04 04`.
PKCS7 ensures that plaintext lengths align with the cipher’s block size, but improper validation of padding can lead to padding oracle attacks.

## Wireguard
WireGuard is a modern VPN protocol and implementation designed by Jason A. Donenfeld.
It focuses on simplicity, performance, and a small codebase, in contrast to traditional VPN stacks like IPsec and OpenVPN.
WireGuard uses a fixed, opinionated set of strong primitives (e.g., Curve25519, ChaCha20-Poly1305, BLAKE2s, and HKDF) arranged in a Noise-based handshake pattern.
It is now integrated into the Linux kernel and widely deployed for site-to-site VPNs, remote access, and privacy-focused applications.

## Non-AES Hardware
“Non-AES hardware” refers to CPUs and devices that do not provide hardware acceleration for AES (such as Intel’s AES-NI instructions).
On these platforms, software-only AES can be relatively slow and sometimes harder to implement in a constant-time way.
Stream ciphers like ChaCha20, which use simple add-rotate-xor operations, tend to be faster and easier to harden against timing attacks on such hardware.
Modern protocols therefore often select ChaCha20-Poly1305 as a preferred or fallback cipher suite for clients without efficient AES support.

## Salsa20
Salsa20 is a stream cipher designed by Daniel J. Bernstein as part of the eSTREAM project.
It uses simple add-rotate-xor (ARX) operations to generate a pseudorandom keystream from a key, nonce, and counter, making it fast and easy to implement on a wide range of hardware.
Salsa20 was extensively analyzed and gained a strong security reputation, and its design directly inspired ChaCha20, which refines the permutation for better diffusion and performance on modern CPUs.

## Pseudorandom Keystream
A pseudorandom keystream is a sequence of bits or bytes that appears random but is generated deterministically from a secret key (and often a nonce/counter).
In a secure stream cipher, this keystream should be computationally indistinguishable from true randomness to anyone who does not know the key.
When the keystream is XORed with plaintext (and never reused with the same key/nonce), it hides the original data while still allowing the receiver, who can regenerate the same keystream, to decrypt it.

## ChaCha20-Poly1305
ChaCha20-Poly1305 is an AEAD (Authenticated Encryption with Associated Data) construction that combines the ChaCha20 stream cipher with the Poly1305 message authentication code.
ChaCha20 provides fast, software-friendly encryption using add-rotate-xor operations, while Poly1305 computes a strong one-time MAC over the ciphertext and associated data.
Together, they provide confidentiality, integrity, and authenticity in a single primitive, widely deployed in TLS 1.3, WireGuard, and many modern protocols, especially on devices without AES hardware acceleration.

## Salt
A salt is a random value added to a password before hashing or key derivation to ensure that identical passwords never produce identical outputs. It completely defeats precomputed attacks such as rainbow tables by forcing attackers to recompute hashes for every individual target.
Salts are not secret and are stored alongside the hash, but they must be unique and random per password. A salt does not slow down brute force by itself—it only prevents large-scale hash reuse and precomputation.

## PBKDF2
PBKDF2 is a standardized function used to derive cryptographic keys from passwords by applying a hash function (via HMAC) many times in succession to slow down brute-force attacks.
It is widely supported, easy to implement, and still secure when configured with a high iteration count, but it is not memory-hard, making it vulnerable to modern GPU and ASIC attacks. Today, it is considered legacy-safe but no longer state-of-the-art.

## Argon2
Argon2 is the modern standard for password hashing and key derivation, designed to resist GPU, FPGA, and ASIC attacks through memory-hard computation. It won the Password Hashing Competition in 2015.
It allows precise tuning of memory usage, execution time, and parallelism, making large-scale cracking physically expensive. The recommended variant today is Argon2id, which combines security against side-channels and brute-force attacks.

## scrypt
scrypt is an earlier memory-hard password-based key derivation function created to make hardware attacks costly by requiring large amounts of memory during computation.
It remains cryptographically strong and widely used, especially in cryptocurrencies and disk encryption, but it is harder to tune correctly than Argon2 and is now generally considered second-choice for new systems.

## HKDF
HKDF is a key derivation function used for expanding a strong secret into multiple independent cryptographic keys using HMAC. It is designed for key separation and protocol key management, not for passwords.
It is heavily used in TLS, secure messaging, and operating systems. HKDF provides no brute-force protection and must never be used directly with human passwords.
