## Formal Security Notions & Proof Intuition

> 🔐 **Used in:** TLS, Signal protocol, Noise protocol framework, modern AEAD schemes, authenticated key exchange protocols, academic cryptography

> ✅ Formal security notions define *what it means for a cryptographic system to be secure* and allow researchers to prove that protocols resist well-defined classes of attacks.

Cryptography is not only about algorithms.

It is about provable security guarantees.

Modern cryptographic systems are designed with formal definitions of security, allowing researchers to demonstrate that breaking a protocol would require solving a mathematically hard problem.

Instead of saying *“this seems secure”*, formal cryptography asks:

- What exactly does “secure” mean?
- What capabilities does the attacker have?
- What advantage must the attacker gain to break the system?

Security proofs do not guarantee absolute safety.

They guarantee that if the underlying assumptions hold, then the system resists a defined class of attacks.

## The Core Idea: Security Games

Most modern cryptographic definitions are expressed using security games.

A security game defines an interaction between:

- a challenger (the system)
- an adversary (the attacker)

The attacker attempts to win the game by breaking some property.

Example intuition for encryption security:

1. The attacker submits two messages.
2. The challenger encrypts one of them randomly.
3. The attacker must guess which one was encrypted.

If the attacker cannot guess better than random chance, the scheme is considered secure under that definition.

Security is therefore defined as: the attacker has only negligible advantage over random guessing.

## Indistinguishability (IND)

One of the most important security notions is indistinguishability.

The idea is simple:

> An attacker should not be able to distinguish the encryption of one message from another.

This property is formalized as IND security.

Common variants include:

- IND-CPA — secure against chosen plaintext attacks
- IND-CCA — secure against chosen ciphertext attacks

These definitions model increasingly powerful attackers.

### IND-CPA (Chosen Plaintext Attacks)

Here, we assumes the attacker can:

- request encryptions of messages of their choice
- observe ciphertexts
- attempt to distinguish encrypted messages

A scheme that passes this test prevents attackers from learning information about plaintexts from ciphertexts.

Most modern symmetric encryption schemes aim for IND-CPA security.

However, IND-CPA alone is not sufficient in many real-world protocols.

### IND-CCA (Chosen Ciphertext Attacks)

It models stronger attackers.

In this game, the attacker can:

- request encryptions
- request decryptions of arbitrary ciphertexts
- attempt to distinguish the challenge ciphertext

The only restriction is that the attacker cannot request the decryption of the challenge ciphertext itself.

This models realistic scenarios where attackers can trick systems into decrypting messages.

Modern authenticated encryption schemes aim for IND-CCA security.

AEAD constructions such as AES-GCM and ChaCha20-Poly1305 are designed to resist these attacks.

## Message Authentication Security (EUF-CMA)

For digital signatures and MACs, the typical security notion is existential unforgeability under chosen message attack (EUF-CMA).

In this model:

1. The attacker can request signatures on arbitrary messages.
2. The attacker wins if they produce a valid signature for a *new* message.

This ensures attackers cannot forge signatures even if they observe many valid ones.

Most modern signature schemes such as Ed25519 are designed to satisfy EUF-CMA security.

## Negligible Probability

Formal cryptography frequently uses the concept of negligible probability.

A negligible function decreases faster than the inverse of any polynomial.

In practical terms this means:

- an attack success probability so small that it becomes infeasible
- even for extremely large computational resources

Example intuition:

If a scheme has 128-bit security, brute force attacks require roughly:

2¹²⁸ operations.

This is considered infeasible with any realistic computing capability.

Security proofs often show that an attacker's advantage is bounded by a negligible probability.

## Reduction Proofs

Most cryptographic proofs are reduction proofs.

The idea is:

> If an attacker can break the protocol, then we can use that attacker to break a known hard problem.

For example, breaking a scheme implies solving:
- discrete logarithm
- factoring
- elliptic curve discrete logarithm

Since those problems are believed to be computationally hard, the scheme is considered secure.

Reduction proofs therefore connect protocol security to underlying mathematical assumptions.

## Proof Models

Formal security proofs are performed in specific models.

Two major approaches dominate modern cryptography.

### The Standard Model

The **standard model** assumes only well-defined mathematical hardness assumptions.

Examples include:

- discrete logarithm hardness
- factoring hardness
- elliptic curve discrete logarithm

Proofs in the standard model are considered the strongest form of security guarantees.

However, they can be difficult to achieve.

### The Random Oracle Model

Many practical cryptographic schemes rely on the random oracle model.

In this model:

- hash functions are treated as ideal random functions
- attackers interact with this oracle during the security game

This abstraction simplifies proofs and enables efficient constructions.

Many widely deployed protocols rely on proofs in the random oracle model.

Examples include:

- RSA-OAEP
- many signature schemes
- parts of TLS

While slightly weaker than the standard model, the random oracle model is widely accepted in practice.

## Why Formal Proofs Matter

Formal security definitions provide several benefits.

They:

- eliminate vague security claims
- define attacker capabilities precisely
- enable rigorous analysis
- allow comparison between schemes

Many historical cryptographic failures occurred before formal analysis became common.

Modern protocol design increasingly relies on provable security frameworks.

However, proofs only apply to the model, not necessarily to the implementation.

Side-channel attacks, bugs, or misuse can still break secure constructions.

## Proofs vs Reality

Even perfectly proven cryptographic constructions can fail in practice.

Reasons include:

- implementation bugs
- side-channel leakage
- incorrect randomness generation
- protocol misuse
- key management failures

Formal proofs ensure that the *design* is secure.

Secure systems still require careful engineering.

> **🟢 Conclusion**
>
> Formal security notions provide the foundation for modern cryptography:
>
> - Security games define attacker capabilities.
> - Indistinguishability formalizes encryption security.
> - EUF-CMA defines signature security.
> - Reduction proofs connect attacks to hard mathematical problems.
> - Security models like the standard model and random oracle model structure formal analysis.
>
> These tools allow cryptographers to reason rigorously about security, ensuring that protocols are not only intuitive but mathematically sound.
