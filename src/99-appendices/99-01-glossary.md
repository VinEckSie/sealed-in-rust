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

## WireGuard
WireGuard is a modern VPN protocol and implementation designed by Jason A. Donenfeld.
It focuses on simplicity, performance, and a small codebase, in contrast to traditional VPN stacks like IPsec and OpenVPN.
WireGuard uses a fixed, opinionated set of strong primitives (e.g., Curve25519, ChaCha20-Poly1305, BLAKE2s, and HKDF) arranged in a Noise-based handshake pattern.
It is now integrated into the Linux kernel and widely deployed for site-to-site VPNs, remote access, and privacy-focused applications.

## Non-AES Hardware
“Non-AES hardware” refers to CPUs and devices that do not provide hardware acceleration for AES (such as Intel’s AES-NI instructions).
On these platforms, software-only AES can be relatively slow and sometimes harder to implement in a constant-time way.
Stream ciphers like ChaCha20, which use simple add-rotate-xor operations, tend to be faster and easier to harden against timing attacks on such hardware.
Modern protocols therefore often select ChaCha20-Poly1305 as a preferred or fallback cipher suite for clients without efficient AES support.

## AEAD
AEAD (Authenticated Encryption with Associated Data) is a class of symmetric encryption schemes that provide confidentiality and integrity together.
An AEAD encrypts plaintext into ciphertext and simultaneously produces an authentication tag which is verified during decryption.
It also supports authenticating “associated data” (AAD): metadata such as headers or protocol fields that must be integrity-protected but remain unencrypted.
If tag verification fails, decryption must fail and no plaintext should be released. Common AEADs include AES-GCM and ChaCha20-Poly1305.

## AEAD API
An “AEAD API” is the standard programming interface exposed by AEAD libraries: given a key, nonce, plaintext, and optional AAD, it returns ciphertext plus an authentication tag.
Decryption takes the same inputs and returns either the plaintext or an authentication failure, typically as an error result.
Most APIs treat the nonce as public but unique, and require callers to enforce nonce rules; many also offer “in-place” variants for performance.
Using the API correctly means never decrypting unauthenticated data and always handling verification errors as hard failures.

## Misuse Resistance
Misuse resistance is a property of cryptographic constructions that reduces the damage caused by common implementation mistakes.
In AEADs, the most important misuse is nonce reuse: many schemes (like AES-GCM) fail catastrophically if a nonce is repeated under the same key.
Misuse-resistant AEADs (such as AES-GCM-SIV) are designed so that accidental nonce reuse does not immediately enable forgeries or full plaintext recovery.
Misuse resistance is not a license to be sloppy—reusing nonces can still leak information—but it provides a safety net for real systems.

## TLS Fallback
TLS fallback refers to selecting an alternative cipher suite or algorithm when the preferred choice is unavailable or performs poorly on a given client.
In modern TLS, clients and servers negotiate a suite during the handshake; implementations may prefer AES-GCM on CPUs with AES acceleration and prefer ChaCha20-Poly1305 on devices without it.
This is typically a performance and side-channel hardening decision, not a downgrade in security, as both suites are considered strong when used correctly.
The key point is that TLS can adapt to heterogeneous hardware while maintaining authenticated encryption.

## AES Block Cipher
AES is a block cipher: it transforms fixed-size 128-bit blocks under a secret key.
By itself, AES does not define how to encrypt messages longer than one block or how to authenticate data; those properties come from a “mode of operation” and/or an AEAD construction.
Thinking “AES” vs “AES-GCM” is the difference between the raw primitive (block-by-block permutation) and a complete, safe encryption scheme.
Most real-world systems use AES inside a mode such as CTR, CBC, or an AEAD like GCM, rather than using AES directly on blocks.

## CTR Mode
CTR (Counter) mode turns a block cipher like AES into a stream-cipher-like construction by encrypting a nonce/counter sequence to generate a keystream.
That keystream is XORed with plaintext to produce ciphertext (and vice versa for decryption).
CTR mode is fast and parallelizable, but it provides no integrity: it is malleable, and attackers can flip bits in the decrypted plaintext by flipping bits in ciphertext.
Nonce/counter values must never repeat under the same key or the keystream repeats and confidentiality breaks.

## GHASH
GHASH is the authentication component used by AES-GCM.
It is a polynomial hash over GF(2^128) computed on the associated data and ciphertext, producing a value that is combined with an AES-derived mask to form the final tag.
GHASH is efficient and works well with hardware acceleration, but it inherits AES-GCM’s strict nonce-uniqueness requirement: repeating a nonce under the same key can reveal information and enable forgeries.
In practice, you treat GHASH as an internal detail of GCM and focus on correct key and nonce management at the API boundary.

## AES-GCM
AES-GCM (Galois/Counter Mode) is a widely standardized AEAD which combines AES in CTR mode for encryption with GHASH for authentication.
It is extremely fast on modern CPUs with AES acceleration and is common in TLS, HTTPS, storage systems, and many enterprise protocols.
AES-GCM requires a unique nonce for every encryption under the same key; repeating a nonce can reveal relationships between plaintexts and may enable tag forgery.
Because of that, AES-GCM is best used with reliable nonce generation (counters, sequence numbers, or carefully managed randomness) and strict error handling on authentication failure.

## ChaCha20
ChaCha20 is a modern stream cipher designed by Daniel J. Bernstein as a refinement of Salsa20.
It uses simple add-rotate-xor (ARX) operations, making it fast in software and relatively easy to implement in constant-time.
ChaCha20 generates a pseudorandom keystream from a 256-bit key, a nonce, and a counter; encryption XORs this keystream with the plaintext.
Nonce reuse under the same key causes keystream reuse and breaks confidentiality, so nonces must be unique.

## Poly1305
Poly1305 is a fast one-time message authentication code (MAC) designed by Daniel J. Bernstein.
It computes a 16-byte authentication tag using arithmetic modulo a large prime, and it is designed to be implemented efficiently and in constant time.
The crucial rule is that its key must be used only once; reusing a Poly1305 key can allow attackers to forge tags.
In practice, Poly1305 is paired with a stream cipher (most commonly ChaCha20) which derives a fresh one-time key for each message.

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
Together, they provide confidentiality, integrity, and authenticity in a single primitive, widely deployed in TLS 1.3, WireGuard, and many modern protocols.
ChaCha20-Poly1305 is particularly attractive on clients without fast AES hardware because it is performant in pure software and easier to harden against timing leakage than table-based AES implementations.
Like other AEADs, it requires unique nonces per key; nonce reuse can reveal relationships between plaintexts and must be avoided.
Many libraries expose it through a uniform AEAD API, making it a common “safe default” choice across servers, mobile devices, and embedded systems.

## Cache Timing
Cache timing is a side-channel risk where an attacker learns secrets by observing how long computations take due to CPU cache effects.
If an algorithm’s memory access pattern depends on secret data (for example, table lookups indexed by key bytes), the CPU may fetch different cache lines and create measurable timing differences.
These differences can be exploited locally (or sometimes remotely) to recover keys or other sensitive state.
Mitigations include constant-time implementations, avoiding secret-dependent table lookups, using hardware-accelerated instructions (e.g., AES-NI), and careful microarchitectural hardening.

## Side-Channel Attack
A side-channel attack breaks security by exploiting information leaked by an implementation rather than weaknesses in the underlying math.
Common side channels include timing, cache behavior, power consumption, electromagnetic emissions, and fault injection.
For example, a timing difference in MAC verification or a secret-dependent memory access pattern can leak bits of a key over many observations.
Defenses include constant-time code, uniform error handling, blinding techniques, hardened hardware, and minimizing attacker observability.

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

## SHA-2 (Secure Hash Algorithm 2)
SHA-2 is a family of cryptographic hash functions standardized by NIST as successors to SHA-1.
It includes SHA-224, SHA-256, SHA-384, and SHA-512, which differ mainly in output size and internal parameters.
SHA-2 is designed to provide strong collision resistance, preimage resistance, and avalanche behavior.
It is widely used in TLS, digital signatures, certificate authorities, blockchains, and secure software distribution.
Despite its age, SHA-2 remains secure and is considered a conservative, well-understood choice.

## BLAKE3
BLAKE3 is a modern cryptographic hash function derived from the BLAKE hash family, designed for high performance, parallelism, and simplicity.
It builds on the cryptographic foundations of BLAKE2 while introducing a tree-based structure that enables efficient multicore and SIMD execution.
BLAKE3 supports incremental hashing, keyed hashing, and extendable output (XOF) within a single unified API.
It is well suited for content hashing, integrity verification, and high-throughput systems, though it is not standardized by NIST.
BLAKE3 is considered secure and is increasingly adopted in modern software systems.

## NIST
NIST is a U.S. federal agency responsible for developing and publishing technical standards, including cryptographic algorithms.
It standardizes widely used primitives such as AES, SHA-2, RSA padding schemes, and digital signature algorithms.
NIST standards are often required for government, defense, financial, and regulated industries.
While NIST does not “invent” most algorithms, its approval process heavily influences global cryptographic adoption.
Compliance with NIST standards is often referred to as “FIPS compliance.”

## SHA-224
SHA-224 is a cryptographic hash function belonging to the SHA-2 family.
It produces a 224-bit digest and is derived from the same internal structure as SHA-256, with different initialization constants and truncation.
SHA-224 offers slightly reduced output size and security margin compared to SHA-256.
It is used in constrained environments where smaller hash outputs are desirable, though SHA-256 is far more common.

## SHA-256
SHA-256 is the most widely deployed member of the SHA-2 hash family.
It produces a 256-bit digest and provides strong collision and preimage resistance.
SHA-256 is used extensively in TLS, digital signatures, blockchains (including Bitcoin), and secure software verification.
Its balance of security, performance, and broad support makes it the default hash function in many systems.

## SHA-384
SHA-384 is a SHA-2 variant optimized for 64-bit architectures.
It produces a 384-bit output and uses different internal parameters than SHA-256, providing a higher security margin.
SHA-384 is often used in high-security or long-term systems where additional safety margin is desired.
It is commonly paired with 64-bit platforms and high-assurance protocols.

## SHA-512
SHA-512 is the largest-output member of the SHA-2 family, producing a 512-bit hash.
It is optimized for 64-bit processors and often performs better than SHA-256 on such systems.
SHA-512 provides an extremely large security margin against collision and preimage attacks.
Variants such as SHA-512/256 reuse the SHA-512 core while producing shorter outputs.

## KDF
A Key Derivation Function (KDF) is a cryptographic function that derives one or more cryptographically strong keys from an initial secret.
The input may be a human password, a shared secret, or another cryptographic key.
KDFs enforce key separation, prevent key reuse, and ensure derived keys have the correct length and distribution.
Some KDFs (e.g., Argon2, scrypt) are designed to resist brute-force attacks, while others (e.g., HKDF) are used for safe key expansion.
Choosing the correct KDF depends on whether the input secret is weak or already strong.

## SIMD
SIMD is a CPU execution model where a single instruction operates simultaneously on multiple data elements.
It enables data-parallel computation using vector registers, significantly improving performance for cryptographic and numeric workloads.
Modern CPUs use SIMD extensively for hashing, encryption, and multimedia processing.
Algorithms designed with SIMD in mind (such as BLAKE3) can achieve very high throughput on modern hardware.

## Incremental Hashing
Incremental hashing is the ability to compute a hash progressively by processing input data in chunks.
This allows hashing of large or streaming data without loading it entirely into memory.
Incremental hashing is essential for file hashing, network protocols, and real-time data processing.
Most modern cryptographic hash functions support incremental hashing by design.

## Keyed Hashing
Keyed hashing is a hashing mode that incorporates a secret key into the hash computation.
It is used to provide message authentication and integrity, ensuring that only parties with the key can generate or verify the hash.
Keyed hashing prevents attackers from forging valid hashes for modified messages.
HMAC is the most widely used standardized construction for keyed hashing.

## Extendable Output Function
An Extendable Output Function (XOF) is a hash construction that can generate an arbitrary-length output from a single input.
Instead of producing a fixed-size digest, an XOF acts like a pseudorandom stream derived from the input.
XOFs are useful for key derivation, randomness expansion, and domain separation.
Examples include SHAKE and the XOF mode of BLAKE3.

## Collision
A collision occurs when two distinct inputs produce the same hash output.
Collision resistance means it is computationally infeasible to find such pairs intentionally.
While collisions must exist mathematically, a secure hash makes finding them impractical.
Collision resistance is critical for digital signatures, certificates, and integrity systems.

## Brute Force
A brute-force attack is an exhaustive search that tries all possible inputs or keys until the correct one is found.
Its effectiveness depends entirely on the size of the search space and the speed of computation.
Modern cryptography relies on making brute-force attacks computationally infeasible.
KDFs and memory-hard functions are designed specifically to slow down brute-force attacks.

## Preimage
A preimage is an input that produces a specific hash output.
Preimage resistance means it is computationally infeasible to recover the original input from its hash.
This property ensures that hashes cannot be reversed to reveal sensitive data.
Preimage resistance is fundamental for password hashing and data integrity.

## Merkle Tree
A Merkle tree is a hash-based tree data structure where each internal node is the hash of its child nodes.
It allows efficient verification of large data sets by validating only a small subset of hashes.
Merkle trees are used in blockchains, distributed systems, file systems, and secure databases.
They provide tamper detection and integrity guarantees with logarithmic verification cost.

## Forged API Request
A forged API request is a maliciously crafted or modified request designed to impersonate a legitimate client or bypass security controls.
Attackers may manipulate headers, parameters, identifiers, signatures, or timestamps to gain unauthorized access or perform restricted actions.
These attacks often exploit weak authentication, missing authorization checks, predictable identifiers, or insufficient request validation.
Mitigations include strong authentication, strict authorization, request integrity verification, and anti-replay mechanisms.

## Token Manipulation
Token manipulation refers to tampering with authentication or session tokens to escalate privileges or impersonate another identity.
Common targets include JWTs, cookies, and API keys where claims, signatures, or metadata may be altered or abused.
Vulnerabilities often arise from weak verification, algorithm confusion, excessive token lifetime, or insecure storage.
Defenses include strict signature validation, short expirations, audience checks, and secure token handling practices.

## Padding Oracle Attack
A padding oracle attack exploits observable differences in error messages or timing when a system validates decrypted padding.
By sending many crafted ciphertexts, an attacker can progressively recover plaintext or forge valid encrypted messages.
This attack commonly affects block cipher modes such as CBC when padding errors are distinguishable.
Using authenticated encryption, uniform error handling, and constant-time validation prevents this class of attack.

## Silent Data Corruption
Silent data corruption occurs when data is modified without detection due to hardware faults, software bugs, or transmission errors.
Because no immediate error is raised, corrupted data may propagate into backups, computations, or decision systems.
This can silently compromise integrity, correctness, and long-term reliability of stored or processed information.
End-to-end checksums, cryptographic integrity verification, and hardware redundancy reduce this risk.

## JWT
A JSON Web Token is a compact, URL-safe token format used to transmit cryptographically protected claims between parties.
It typically consists of a header, payload, and signature encoded using Base64URL.
JWTs are widely used for stateless authentication and authorization in distributed systems and APIs.
Secure usage requires strict signature verification, algorithm allow-lists, expiration checks, and issuer validation.

## API
An application programming interface defines how software components communicate through structured requests and responses.
It specifies operations, data formats, authentication rules, error handling, and versioning conventions.
APIs enable modular system design, interoperability, and automated integration between services.
Security depends on proper identity verification, authorization enforcement, input validation, and transport protection.

## OAuth 2.0
OAuth 2.0 is an authorization framework that enables delegated access to protected resources without sharing user credentials.
It allows applications to obtain limited-scope access tokens issued by an authorization server.
OAuth separates authentication from authorization and supports multiple standardized authorization flows.
Secure deployments rely on PKCE, strict redirect validation, token verification, and safe client storage.

## AWS Request Signin (SigV4)
AWS request signing is a cryptographic mechanism used to authenticate and authorize API requests to AWS services.
Requests are canonicalized, hashed, and signed using HMAC with secret credentials and request metadata.
This protects against request tampering and limits replay attacks through timestamp validation.
Correct canonicalization and key handling are critical for reliable and secure operation.

## TLS Internals
TLS internals describe the cryptographic protocols and handshake mechanisms that secure network communications.
During the handshake, peers authenticate, negotiate algorithms, and derive shared symmetric session keys.
Modern TLS provides confidentiality, integrity, authentication, and forward secrecy.
It protects application data against eavesdropping, tampering, and active network attacks.

## Length-Extension Attack
A length-extension attack exploits the internal structure of certain hash functions to append data to a hashed message.
An attacker can compute a valid hash for an extended message without knowing the original secret.
This vulnerability breaks naïve constructions such as hash-based authentication using secret-prefix hashing.
Proper MAC constructions such as HMAC are specifically designed to prevent this attack.

## Constant-Time Algorithm
A constant-time algorithm executes independently of secret data to avoid leaking information through timing behavior.
Branching, memory access patterns, and early exits must not depend on sensitive values.
Timing side channels can otherwise reveal keys, authentication tags, or internal state.
Constant-time techniques are essential for secure cryptographic implementations.

## Stream Cipher
A stream cipher encrypts data by generating a pseudorandom keystream and XORing it with plaintext bytes.
The same operation is used for encryption and decryption.
Reusing the same keystream for multiple messages compromises confidentiality immediately.
Modern stream ciphers are typically combined with authentication to provide full data integrity.
