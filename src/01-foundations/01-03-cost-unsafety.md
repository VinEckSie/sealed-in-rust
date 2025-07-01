## The Cost of Unsafety in Crypto: Famous Failures

Cryptography doesn’t fail because math is broken — it fails because **systems leak**, **code panics**, or **side-channels whisper secrets**.<br><br>
And most of these failures? They stem from unsafety.
<br><br><br>
Here are just a few real-world examples of cryptographic disasters caused by unsafe programming, undefined behavior, or lack of control:

#### Heartbleed (2014)
- **Cause:** Buffer over-read in OpenSSL (written in C)
- **Impact:** Leaked private keys, passwords, and session data from millions of servers
- **Lesson:** Unsafe memory access can silently expose secrets

#### Debian RNG Bug (2006–2008)
- **Cause:** A developer commented out entropy-gathering code in OpenSSL
- **Impact:** Generated only 32,768 possible SSH keys across all Debian systems
- **Lesson:** Cryptographic quality often hinges on **deterministic, auditable behavior**

#### Lucky13 Attack (2013)
- **Cause:** Tiny timing differences in CBC mode padding checks (TLS)
- **Impact:** Allowed attackers to decrypt data by measuring how long responses took
- **Lesson:** Timing leaks can **invalidate encryption**, even with perfect math

#### JavaScript Crypto Fails
- **Cause:** Misuse of `Math.random()` or insecure key handling in frontend apps
- **Impact:** Predictable keys, insecure password storage, and non-constant-time comparisons
- **Lesson:** Languages with hidden optimizations make **constant-time logic fragile**

### Why Rust Helps

Rust’s safety model eliminates whole classes of vulnerabilities:
- No null/dangling pointers
- No uninitialized memory
- Memory-safe concurrency
- Deterministic behavior at runtime (no GC pauses, no JIT surprise)

You still have to **design crypto carefully**, but with Rust, you’re not building it on quicksand.


> Writing secure cryptography in unsafe languages is like writing legal contracts with disappearing ink.
