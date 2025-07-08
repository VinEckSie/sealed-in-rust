## Cryptography is a Systems Problem

Cryptography isn’t just about math.

Yes, it starts with elegant algebra and deep number theory — but where it breaks is almost always *in the system*. Real-world failures come from poor implementations, leaky abstractions, memory bugs, side channels, or simply misunderstanding what problem crypto is supposed to solve.

It’s easy to misuse even “secure” primitives. AES[^aes] in ECB mode[^ecb] is fast — and useless. RSA[^rsa] without padding[^padding] is a gift to attackers. And a perfectly strong key means nothing if it’s printed to your logs.

This is why cryptography is a **systems engineering problem** first.

And it’s why Rust matters.

Rust doesn’t make crypto correct by default — nothing does — but it gives you tools to **avoid entire classes of catastrophic bugs**. 

Memory safety, explicit ownership, fearless concurrency, and tight control over the machine — these aren’t “nice to have.” They’re security features.
###
In this book, we’ll treat crypto not as a black box, but as a series of concrete systems problems — and show how Rust lets us solve them with clarity and precision.


[^aes]: AES (Advanced Encryption Standard) is a widely used symmetric cipher known for its performance and strong security — when used correctly.
[^ecb]: ECB (Electronic Code Book) mode encrypts each block independently, revealing patterns in the plaintext and making it insecure for most purposes.
[^rsa]: RSA (Rivest–Shamir–Adleman) is a public-key cryptosystem used for encryption and digital signatures. It relies on the difficulty of factoring large integers.
[^padding]: Padding in RSA adds randomness or structure to the plaintext before encryption, preventing deterministic outputs. Without it, attackers can guess messages, detect patterns, or exploit mathematical properties to break the cipher.