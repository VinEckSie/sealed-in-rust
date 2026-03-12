## Secure Protocol Design & Composition

> 🔐 **Used in:** TLS, QUIC[^quic], WireGuard, Noise protocol framework[^noise-protocol-framework], secure messaging, distributed systems
>
> ✅ A secure protocol ensures that cryptographic primitives are composed safely, not just implemented correctly.

Cryptography rarely fails because AES, SHA-256, or Ed25519 are broken.

It fails because protocols compose them incorrectly.

Examples of protocol failures include:

- replayable authentication messages
- nonce reuse in encryption
- downgrade attacks forcing weaker algorithms
- missing state validation
- mixing keys across different purposes

Designing secure protocols means answering:

- Who is the attacker?
- What messages can they observe, modify, or replay?
- Which keys protect which operations?
- What states are valid or invalid in the protocol flow?

Secure protocols are about structure, not just math.

## The Basic Model: primitives → protocol

Cryptographic primitives provide building blocks.
Protocols define how those blocks are used together.

```text
ciphertext = Encrypt(key, plaintext)
tag = MAC(key, message)
signature = Sign(private_key, message)
```

A secure protocol defines:
- who generates keys
- which keys protect which messages
- when keys rotate
- which messages are allowed in which order

If the protocol logic is wrong, perfect cryptography cannot save it.

## Threat Models (define the attacker first)

Before designing any secure protocol, you must define the threat model.

Typical attackers include:
- Passive attacker: can observe traffic but not modify it
- Active attacker: can intercept, modify, or inject messages
- Replay attacker: can resend previously valid messages
- Downgrade attacker: tries to force weaker algorithms

Most real-world protocols assume active network attackers.

This means the attacker can:
- read
- drop
- modify
- reorder
- replay
- inject

If your protocol survives this model, it is usually robust.

## Key Separation (one key, one purpose)

A fundamental cryptographic design rule is key separation.

One key → one purpose.

Bad design is a same key used for:
- encryption
- authentication
- key derivation

Good design:
- enc_key = HKDF(master_key, "encryption")
- mac_key = HKDF(master_key, "authentication")

Why this matters:
- reduces cross-protocol attacks
- prevents subtle cryptographic interactions
- isolates failures if one primitive breaks

Modern protocols like TLS 1.3 and Noise aggressively separate keys.

## Nonce Reuse Disasters

A nonce is a number used once. Many encryption modes require nonce uniqueness like AES-GCM.

If a nonce is reused with the same key: ciphertext1 XOR ciphertext2 = plaintext1 XOR plaintext2.

This can reveal plaintexts or even recover keys in some constructions.

Real-world examples include:
- wireless encryption failures like WEP
- TLS implementation bugs
- application-level nonce reuse

> ✅ Correct rule: nonce must NEVER repeat for the same key

Typical strategies include:
- monotonic counters
- random nonces with large space
- deterministic nonce derivation

## Replay Attacks

A replay attack occurs when an attacker resends a previously valid message.

Example: client → server "transfer 100 coins"

Even if the message is authenticated, replaying it may repeat the action.

Protocols defend against this using:
- sequence numbers
- nonces
- timestamps
- session identifiers

Example defensive pattern:
- message = {sequence_number, payload}
- MAC(key, message)

If the sequence number repeats, the message is rejected.

## Downgrade Attacks

A downgrade attack forces a protocol to use weaker algorithms.

Example negotiation:
- Client: supports AES-256, AES-128
- Attacker modifies → Server sees only AES-128

The attacker may now exploit weaker cryptography. Modern protocols bind negotiation results into the handshake.

Example principle: handshake_signature = Sign(server_key, transcript_hash)

This ensures the final cryptographic choices cannot be altered.

## Protocol Layering

Secure protocols are often structured in layers.

Example TLS model:
Transport > Handshake protocol > Key exchange > Authenticated encryption

Layering improves:
- maintainability
- security reasoning
- replaceability of components

However, layering mistakes can introduce vulnerabilities.

Example historical mistake: compress > then encrypt

Compression-based side channels enabled the CRIME[^crime] and BREACH[^breach] attacks in TLS.

## Protocol State Machines

Protocols are not just message formats. They are state machines.

Example simplified handshake:
START > CLIENT_HELLO > SERVER_HELLO > KEY_EXCHANGE > AUTHENTICATED

Each message is only valid in specific states.
Bad implementations often accept messages in invalid states, enabling attacks.

Common vulnerability classes include:
- state confusion
- unexpected message acceptance
- skipped authentication phases

Robust protocol implementations explicitly enforce state transitions.

## Authenticated Encryption Patterns

Modern secure protocols rely heavily on AEAD.

AEAD provides:
- confidentiality
- integrity
- authentication

Example encryption pattern: ciphertext, tag = AEAD_Encrypt(key, nonce, plaintext, associated_data)

Associated data (AAD) allows binding metadata to encrypted messages.

Example: AAD = message_header

Even though the header is not encrypted, it is authenticated.

This prevents attackers from modifying routing or protocol metadata.

## Common Protocol Design Mistakes

Real-world systems often fail due to protocol composition mistakes:
- Missing replay protection
- Nonce reuse in AEAD
- Algorithm negotiation without authentication
- Key reuse across different purposes
- Improper state validation
- Mixing compression and encryption

Almost all major protocol failures come from composition errors, not primitive failures.

> **🟢 Conclusion**
>
> Secure protocols require more than strong cryptographic primitives:
>
> - They require correct composition.
>
> - Threat models define the attacker.
>
> - Key separation isolates cryptographic roles.
>
> - Nonce uniqueness protects encryption.
>
> - Replay protection defends against message reuse.
>
> - State machines enforce valid protocol flows.
>
> Modern protocols combine these ideas to build systems like TLS, QUIC, and WireGuard.


[^quic]: QUIC: UDP-based transport protocol that integrates TLS 1.3 for security and supports multiplexed streams (used by HTTP/3). [More](../99-appendices/99-01-glossary.md#quic)
[^noise-protocol-framework]: Noise Protocol Framework: framework of handshake patterns for building secure AKE protocols from DH + hashing + AEAD; used by WireGuard. [More](../99-appendices/99-01-glossary.md#noise-protocol-framework)
[^crime]: CRIME attack: TLS compression side-channel attack that leaks secrets by observing compressed sizes. [More](../99-appendices/99-01-glossary.md#crime-attack)
[^breach]: BREACH attack: HTTP response compression side-channel attack that leaks secrets by observing HTTPS response sizes. [More](../99-appendices/99-01-glossary.md#breach-attack)
