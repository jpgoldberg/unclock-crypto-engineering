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
xxd -p  < pt.bin
807060504030201008070605040302
```

That looks tantalizingly to a a PCKS#7 pad.

And thanks to SKaunov's advice to use `-nopad` I can get this to work.

```sh
openssl enc -d -aes-256-ecb -nopad -in ct.bin -K $key > pt.bin
```

% xxd -p < pt.bin
80706050403020100807060504030201


So let me go to the other end of the spectrum. https://docs.rs/aes/latest/aes/struct.Aes256.html

Getting the key and ciphertext from the hex representation to the GenericArrays that that library wanted was a struggle. But after that, I do get a result of 80706050403020100807060504030201

## 3.9

> Using an existing cryptography library, encrypt the following plaintext (in hex)
>
> ```hex
>	29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
> ```
>
with the following 256-bit key (also in hex)
>
> ```hex
>	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
>	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
> ```

The key is the same as for 3.8. I am assuming that this is just encrypting a block, so
effectively ECB is that is intended.

I get 80000000000000000000000000000001


> using *AES*. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

This isn't clear whether I should be encrypting the ascii hex or the raw bytes.
My guess is that it is the ASCII hex so that we can see the results, and the main intent
is to force usage of gpg keygen and other tools.

I don't have an RSA 3072 keypair handy

I used `gpg --full-generate-key` to give me a dialogue that allows me to pick algorithm
and key size.

I did a listing of all my secret keys to find the ID for the one I just created.
It's 88A8ECE213A7B04E. Here is its listing.

```
% gpg --list-keys 88A8ECE213A7B04E
pub   rsa3072 2022-11-29 [SC] [expires: 2023-11-29]
      9819E1AC0EE686B77E3587F888A8ECE213A7B04E
uid           [ultimate] Jeffrey Goldberg (Not for use. Just a class assignment) <jeffrey@goldmark.org>
sub   rsa3072 2022-11-29 [E] [expires: 2023-11-29]
```

I put the output (as bytes, not the ASCII hex) of the AES encryption into a file ex3-9.out

```sh
% echo 80000000000000000000000000000001| xxd -r -p > ex3-9.out 
```

Wow. The GPG/PGP CLI has changed since I last did any of this stuff on the command line.
Ah, I see. The "specify the recipient on the command line in any of a dozen different ways and hope we parse the intentions as you might wish" is deeply unsafe.
I'm glad they got rid of that.

I have encrypted it (with the -a option) and here it is

```
-----BEGIN PGP MESSAGE-----

hQGMA3/zrBp0R3ONAQwAkVYvKxsg6H4wRTYXR7gI0JrLxXydydjTdwlQh4D/pNcW
TdI3I50G6IrjxcVIKxaPIzEKNSlTrHRruBDXLNQM4JZPDnn0HwKpy0mPpU6Pn0K1
Wdg6XzgrPpcoHdNMv9I+G0W24iipa1xpT/FieqlK6U6euX/GoPJ8718cr9wI8g8V
ufsEYBAXfEG9IRzZE6FyK+/aiMby1xsbJS6RJLMbSwkdAf/6iXHwpt+AuPgm6hLz
dX3hCOGDBvrw4vw+B0sdZIeN1ypkPA4eUNMvO80VgdWQKWRcF2s2RPuDoRG5aizE
VEj90xVE8JIX45oDQyC54ACRz/vFaJ6umeVJnnaGLiM49C2503tOpWaS9MVWma8A
9Ga3kyjIsVCLOIadfVCXz4lXPD5bxKUSHbl5vv2CUDG5CM7mSyPRcPLpALaKOkFb
hClUoT1obwP7x+aJl9m9flNQnttxhozwSqh3LnbeCbca0exz02q4bOPTTMLEcf+J
xjfgmD0yEA7K725/ttHP0kkBbM5Qqz6p8vDAV4oYNOr0dJNCg0xKxkEQbGoagM7O
+Zu39RPpvX6A3r5hS8UCzVKRts/zjGC2cBl64LqGe/SxCqXdHAKvx4Aj
=EZFM
-----END PGP MESSAGE-----
```

Decrypted and got back the original

```
% xxd -p < gpg-decrypted
80000000000000000000000000000001
```

Lesson: Every bad CLI decision you make, particularly for something that will be scripted, is going to be a problem for people many years later.
I'm not saying that I would have done a better job in the 1980s, but this is excruciating.
And to think that in the 90s, I seriously tried to get lots of people to use this.
