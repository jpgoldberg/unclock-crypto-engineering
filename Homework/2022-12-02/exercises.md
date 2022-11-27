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

## Q 3.8

> Familiarize yourself with a cryptographic CLI tools. A popular open source package is [*OpenSSL*](https://docs.rs/openssl/latest/openssl/aes/index.html).

Been there. Done that. Still I need to look at the documentation each time.

> Using an existing cryptographic library, decrypt the following ciphertext (in hex)
>
> ```
> 	53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
> ```
>
> with the following 256-bit key (also in hex)
>
> ```
>	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
>	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
> ```
>
> using *AES*.

Given that the the ciphertext is exactly 1 AES block, this must be ECB.

Get the ciphertext into a variable

```sh
ct=$(echo -n "53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07" | tr -d  ' ') 
```

And the same with the key

```sh
raw_key="80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01"
key=$(echo $raw_key | tr -d '\n ')
```

Now to look up the OpenSSL CLI arguments for this.

The algorithm will be `-aes-256-ecb`. 

Hmm. While it takes the key as a hex string, I need to actually get the cipher text to raw bytes. So 

```sh
echo -n $ct | xxd -r -p > ct.bin
```

Decryption should be 

```sh
openssl enc -d -aes-256-ecb -in ct.bin -K $key > pt.bin
```

Let me see if that works

Well fooey. The that gives me 15 bytes of output.

```sh
xxd -p  < pt.bin       (main)unclock-crypto-engineering
807060504030201008070605040302
```

That looks tantalizingly to a a PCKS#7 pad.

Anyway, I am coming to suspect that I shouldn't be doing with on the CLI.

If Brain Smith exposed ECB in ring, I am sure he had a good reason. If not I will look at the Rust openssl bindings

Good for Brian Smith and ring. Ring is suitably high level and opinionated. It doesn't expose such primitives.

So let me go to the other end of the spectrum. https://docs.rs/aes/latest/aes/struct.Aes256.html

Getting the key and ciphertext from the hex representation to the GenericArrays that that library wanted was a struggle. But after that, I do get a result of 80706050403020100807060504030201

I do not know why I am losing that last byte when I use openssl at the command line.