---
tags: uncloak-cesg
---

# 2022-11-18 Session 1 Notes

## Talk about 
- Structure
    - Roughly following the book, leeaning over into other resources as necessary, with recommendations for extra reading in the homework
    - The approach of a study group is different than that of a class: make connections, ask questions and support one another. Those who ask attempt the exercises and are unafraid to ask dumb questions will get the most out of this.
    - Dumb-but-honest questions > showing off how smart you are questions. The best way to show how smart you are is to help other people and answer questions.
    - Homework: Each week will have 1-2 hours of reading homework, and 3-5 hours of additional exercises for those who have the time. We will post solutions to the exercises at the end of each week. Use the group! Ask questions, and work with others if you would like.
- Book
    - “Cryptography is the art and science of encryption. At least, that is how it started out. Nowadays it is much broader, covering authentication, digital signatures, and many more elementary security functions.” We feel much the same way. Cryptography has grown a lot in the last 12 years since the book's publishing. We'll be using the book as a starting point, and suggest supplementary resources where it seems appropriate.
- Rust
    - Every 5 weeks is scheduled code review week, where every student who wants to can submit a piece of code. I'll be reviewing alongside some other cryptography engineers who have volunteered to help with that. 
- Groups
Prepped message:
**If you intend to do at least some of the homework and would like to be part of a smaller study group**, please react to this message with **a single** emoji. 

Others who react with the same emoji as you are your **homework buddy group**; start a group chat with your buddy group to discuss the homework. Please try to **upper-limit groups to 5 members max**. 

That is, **if your emoji reaction has more than 5, consider moving to another emoji** 🥒.

- On the [Uncloak](https://uncloak.org) graph
    - This server has two projects, both in service of making cryptography accessible. First is the study group, where we're attempting to address the need for better group resources on Rust, cryptography, and cryptography engineering. Second is the graph, where we're experimenting with an approach to wiki construction, taking advantage of an interactive graph to demonstrate how concepts relate and build on one another. 

## Exercises
Ch 1:
- Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

Ch 2:
- Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.
- Q4. Suppose Bob receives a messages signed using a digital signature scheme with Alice's secret signing key. Does it prove that Alice saw the message and chose to sign.
- Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
- Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.

General:
- Suppose you read about RSA encryption and wanted to find it's standard specification. Where would you look?
- Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
    - https://cryptography.rs/
    - https://lib.rs/ (librs is equivalent to crates.io, with a different interface)
- Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion). 
- You're implementing a [Tweakable Encryption](https://en.wikipedia.org/wiki/Disk_encryption_theory) scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.
- You want to understand a paper on a new polynomial commitment scheme, but you've been trying for more than an hour, and the math is over your head. What do you do?
- Implement the [Vignère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.
- What is a side channel attack? Is your cipher implementation constant time?
- Extra: Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).
- Extra: Consider ways to contribute what you learned this week to the [Uncloak](https://uncloak.org) knowledge graph.