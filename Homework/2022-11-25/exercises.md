# Exercises

## From the book

### Ch 1. Q10

> Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

### TouchID

When Apple introduced TouchID on iPhones it was understood that fingerprint lifting attacks could be used against it.
While such attacks would be difficult, TouchID added an additional way to unlock a phone.
This clearly adds a weakness. Prior to this, the only way to unlock a locked device was to obtain the victim's passcode.
That attack remains, while now the additional attack of fabricating an appropriate physical finger print is available in certain circumstances.
All prior attacks on locked devices remained; the introduction of TouchID added a new one.

And yet, TouchId was introduced to improve security for millions of users.
Prior to introducing this, most[^1] users did not set a passcode for their device at all.
Setting a passcode enable a wide range of security features that could not be available for devices which did not have a passcode set.
The user-chosen passcode is the only user secret the device has that it can derive other keys from. Local data encryption, for example, requires that the user create a passcode.
Introducing TouchId led to a much larger uptake of setting passcodes.

[^1]: I could look up the numbers Apple reported at the time, but I won't.

### 1Password's secret key

The password manager, 1Password, incorporates a high entropy user secret called the Secret Key into the key derivation process along with the user's account password.
The purpose of this is to make the material stored on the 1Password servers impossible to crack. For more detail see [Secret Key: What is it, and how does it protect you?](https://blog.1password.com/what-the-secret-key-does/).
Note that it does not protect the user from cracking attempts if data is stolen from the user's device.

Loss of the Secret Key leaves the user unable to decrypt their own data. While in some circumstances the is a recovery mechanism, there isn't for individual users who are not part of 1Password teams.
This is an enormous threat to data availability. 

### Ch 2. Q3

> Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.

n(n-1)2 where n = 30. So 15 times 29. So (15 x 20) + (15 x 10) - 15.
So 435.[^435]

[^435]: I thought I could do it more quickly in my head than by using a calculator. That was a mistake.

### Ch2 Q4

> Suppose Bob receives a messages signed using a digital signature scheme with Alice's secret signing key. Does it prove that Alice saw the message and chose to sign.

No. Even under the assumption that only Alice has access to her signing key, she might sign the wrong file.
I, for example, will sign the git commit that contains what I am writing here; but I don't actually know the full format of the commit; `git` is certainly adding stuff to make be a commit, and it could be maliciously tampering with what I am signing.

### Ch2. Q6

> Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?

No. There are attacks on ciphers that do not require key recovery. The malleability of a one time pad is an example. The now famous padding oracle CCA on standard OCB padding is a clear example.

### Ch 2 Q7

> Consider a symmetric-key crypto system in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.

I don't really understand this question. It depends on how many keys will be created this way. So my answer could be 128 (under the assumption that the rest of the crypto system is no weaker than the key).

If, however you are going to be generating anything close to 2^128 keys, then _n_ should be 256.

## General

### RSA standard

> Suppose you read about RSA encryption and wanted to find it's standard specification. Where would you look?

[RFC 8017](https://www.rfc-editor.org/rfc/rfc8017).

### Evaluate libraries

> Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
>
> - https://cryptography.rs/
> - https://lib.rs/ (librs is equivalent to crates.io, with a different interface)

### Benchmark

> Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion).

The documentation is for version 0.3, while the latest version is 0.4.0, which has the cli, statistical analysis, and report generation factored out to cargo-criterion.

This does statistics really nicely with very intelligent sampling,
but it is geared very much toward testing how code changes affect performance.
The statistical comparisons it offers don't really help with identifying side
channel attacks.

Null Hypothesis Significance Tests (with 0.05 threshold) may be right in some contexts, but it is far too conservative for identifying side channels.
A non-significant difference may very well be exploitable.
The tool also comes tantalizingly close to offering statistical comparison between multiple functions, but it doesn't do it.

Perhaps I will see about coding calling the relevant t-tests myself.

### Tweak signature

>  You're implementing a [Tweakable Encryption](https://en.wikipedia.org/wiki/Disk_encryption_theory) scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.

### Hard math

> You want to understand a paper on a new polynomial commitment scheme, but you've been trying for more than an hour, and the math is over your head. What do you do?

A lot depends on how badly I want to understand the paper and at what depth.
There are plenty of circumstances where I will trust the authors and the reviewers that the math does what it says it does.

But I might also

- look through what the paper cites and work backwards;
- pull out my copy of _Introduction to Mathematical Cryptography_ or _Algebra for Cryptologists_;
- go to Wikipedia to get some overview;
- Ask my kid for help, particular if it involves linear algebra.

### Vigenère

> Implement the [Vigenère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.

### Side channel

> What is a side channel attack? Is your cipher implementation constant time?

### Side channel: Key length

In the course of running some other tests, I accidentally ran a test where the key lengths were different.
This resulted an an enormous time difference.
I have yet to test this more systematically,
but given that the “hardest” (well, least easy) step in breaking Vigenère is determining the key length, this, perhaps, suggests that Vigenère is not really suitable for use these days.[^7]

[^7]: In case it isn't obvious, that was deliberate understatement.

### Side channel: Modular Reduction

Whether a modular reduction is needed. Computing `10 + 12 % 26` may take less time than computing `10 + 22 % 26`.

My v0.1.0 was vulnerable to this.
If I encrypt using a key with letters near the beginning of the alphabet, `abcdefg` versus near the end, `tuvwxyz`, I get a measurable difference in the time it takes to encrypt.

![violin plot showing time to encrypt using early or late key types](./violin.svg)

The mean time for when using `abcdefg` is 43.747µs with the 95% confidence interval for the true mean between 43.461µs and 44.103µs.
When using `tuvwxyz`, the mean is 43.783µs (CI: 43.665–43.926µs).
Criterion [doesn't directly compute whether that difference is significant](https://github.com/bheisler/cargo-criterion/issues/4#issuecomment-1325454534),
and I haven't done that math myself, but this really looks like a measurable difference that
leaks information about the letters in the key.

I also tested modular addition directly in cases where the sum is less than the modulus versus cases where it is greater, and I got similar results.

My mitigation was for alphabets whose lengths are powers of 2 to use bitwise masking instead of the modulo[^32] (`%`) operation.
That is where the modulus, `m`, is a power of 2,
I effectively replaced `(a + b) % m` with
`(a + b) & (m-1)`.
The casts needed to get everything to work made the actual code uglier, but that is the concept.
The idea is that the bitwise AND operation will be performed in all cases, even when a + b is less than m.

[^32]: Because we are only dealing with positive operands, I am not going to bother making the distinction between “remainder” and ”modulo” in what I write here.

My results are a bit confusing to me. I am not so much seeing a difference in average time,
but I am seeing less variance in the encrypt time.

![violin chart comparing early/late keys with a 32 character alphabet](./32abc-violin.svg)

I _will_ need to actually run statistics to see how measurable the differences are.

### Side channel: lookup position in String/Vector

Numerical index for a letter might take more time for finding the index at near the end of the string. I attempted to address this by creating a pair of HashMaps to do that lookup instead of searching through a sequence of letters.

## Extra

### New Directions

> Extra: Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).

I should reread that. I haven't (yet).

### Unclock contribution

> Extra: Consider ways to contribute what you learned this week to the [Uncloak](https://uncloak.org) knowledge graph.

Hmm. I probably should see what is already known about modular reductions and side channels.
But I want to think about mitigations prior to reading what the expert community has to say.
  