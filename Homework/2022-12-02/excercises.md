# Homework for December 2, 2022

## Ch 3: 1

> How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?

Each block is 10 bytes, so 2 * 10 * 2^64 bytes.
If you really want to see that in decimal it is `368_934_881_474_191_032_320`.

## Ch2: 5

> Suppose you have a processor that can perform a single DES encryption or decryption operation in $2^{-26}$  seconds. Suppose you also have a large number of plaintext-ciphertext pairs for $DES$ under a single unknown key. How many hours would it take, on average, to find that $DES$ key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of $2^{14}$ processors?

A full search would take _s_ seconds where $s = 2^{-26} \cdot 2^{56} = 2^{30}.
There are approximately[^12] $2^{12}$ seconds per hour, so we are looking at $2^{18}$ hours
for a full search, and so half of that for an average search, yielding $2^{17}$ hours.
For the rest, I need a calculator: Yikes that is around 5500 days (15 years)! That is far longer than I expected.

[^12]: Approximations are good here. We do not want to claim an unjustified precision.

Now with $2^{14}$ processors, our $2^{17}$ hours for the full search becomes $2^{3}$ hours.
So eight hours with all of those processors.

## Ch 3: Q6

> Consider a new block cipher, *DES2*, that consists only of two rounds of the *DES* block cipher. *DES2* has the same block and key size as *DES*. For this question you should consider the *DES* $F$ function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for *DES2* under a single, unknown key. Given an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit *DES* key. Can your algorithm be converted into a distinguishable attack against *DES2*?

I believe that a _Meet_ in the Middle attack should require $2^{49}$ operations. But I should probably read the chapter.
 
