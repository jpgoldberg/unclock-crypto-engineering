# Solutions for homework due December 9

The text of the assignments live in the second have of the [Session 3 Notes](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-02+Session+3+Notes)

## Exercise 5.3

> Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs
> only the first _n_ bits of the result.
> Write a program that uses a birthday attack to find and output a collision on SHA-512-n,
> where n is a multiple of 8 between 8 and 48.
> Your program may use an existing cryptography library.
> Time how long your program takes when n is 16,
> averaged over five runs for each _n_.
> How long would you expect your program to take for SHA-512-256? For SHA-512?

This should be straight forward, though I am not sure whether I will need to save data to disk or whether this can all be done in memory.
But if I am going to store the pre-image that resulted in each hash with the hash,
that will space requirement will grow.

(In my earlier misguided thinking, I though a hashmap had to store key key separately, and so I tried a trie. The name `HashMap` should have clued me in)

I am going to limit my inputs  to $2^{32}$ so that I can store
inputs in a `u32`.
It is a near certainty to have a collision at that point.

For reasons I can no longer recall,
I have a birthday collision approximation calculator in R sitting around.

```R
pbirthday <- function(n, d) {
    exponent <- - (n * (n - 1))/( 2 * d)
    p <- 1 - exp(exponent)
    return(p);
}
```

And a run of it confirms that $2^{32}$ inputs is more than sufficient.

```Rconsole
> pbirthday(2^32, 2^48)
[1] 1
```

The particular approximation is 

$$\begin{equation}
p(n, d) \approx \exp\left(\frac{n(n-1)}{d}\right)
\end{equation}$$

For the smaller hashes we can use the exact formula
$$\begin{equation}
p(n, d) =\prod_{i=1}^{n-1}\left(1- \frac{i}{d}\right)
\end{equation}$$

So my strategy will be to generate 32 bit inputs, and put those inputs as values in a HashMap that is keyed by the truncated hashes.

I have actually added these computations to my code for talking about a found collision. Here is a sample output with an 8-bit hash

```
msg1: 8A2C9267
msg2: 78D156DF
hash: [CD]
After 21 distinct hashes
Going 8.203% of the way through the space
With a 0.57 probability of getting a collision by this point
```

and here is a sample with a 32 bit hash.

```
msg1: 957E5A48
msg2: 2F93C5F6
hash: [F0, D4, 10, 4E]
After 130886 distinct hashes
Going 0.003% of the way through the space
With a 0.86 probability of getting a collision by this point
```

Here is a bad luck (took well above average time) example with the 48-bit hashes.

```
msg1: 527A99B4
msg2: FB587724
hash: [0C, 3F, E8, B1, E4, 4E]
After 38464441 distinct hashes
Going 0.000% of the way through the space
With a 0.93 probability of getting a collision by this point
```

I had spent a lot of time playing with criterion the first week,
so I didn't work on that now. I believe that time and space requirements grow
proportionally with $2^{\ell/2}$ 
where $\ell$ is the length in bits of the truncated hash.
So

$$\begin{equation}
t = k2^{\frac{\ell}2}
\end{equation}$$

Solving for $k$ gives us $k = 2^{-\frac{\ell}2}t$.

Now that I am actually doing time testing under proper
benchmarks instead of as unit tests, I have much faster times and more reliable measurements.

Extrapolating to a hash of 256 and 512 bits doesn't really make sense, as the techniques used so far (in which all of the hashes and their pre-images reside in memory) won't hold. So such extrapolation can only offer a lower bound.

|bits  | time    | k      |
|------|---------|--------|
|  8   | 6.7 µs  | 4.08e-07 |
| 16   | 102 µs  | 3.98e-07 |
| 24   | 1.73 ms | 4.05e-07 |
| 32   | 27.9 ms | 3.77e-07 |
| 48   | 14.7 s  | 8.76e-07 |

So up through 32-bit prefixes we get $k \approx 4\cdot 10^{-7}$ some-weird-unit per second.
It nearly double when we work on 48 bit hashes.
I expect that the discontinuity is due to memory management,
and that lookups in a `HashMap` hit a bump.

So then for the 256-bit hashes we get something like 31 quadrillion ages of the universe,
and for the 512-bit hashes we get $10^{53}$ ages of the universe.
These are not really useful numbers.

Questions that remain for me from this include

1. I still find myself struggling with getting things into and from the types that RustCrypto demands.
Is there a better way of doing what I have here to get my truncated `Vec<u8>`?
   
   ```rust
      let mut hasher = Sha512::new();
      // snip
      loop {
        // snip
        hasher.update(message.to_be_bytes());

        // I feel like there must be a better way of getting my truncated hash
        let hash: Vec<u8> = hasher
            .finalize_reset()
            .iter()
            .take(bytes_to_take.into())
            .copied()
            .collect();
        // snip
      }
   ```

   I feel like there should be a simpler way of getting the first _n_ bytes of what
   `finalize_reset()` returns.
   I will note that this is better than what I first had, but clippy taught me
   about `.copied()` which replaces my `.map(|b| &b)`


## Exercise 5.4

> How long to find a pre-image for a truncated 16 bit hash?

On my system, it is approximately 19 milliseconds.
For a 24 bit hash, about 5.4 seconds.
I will not attempt a 32 bit hash, which should be on the order of 23 minutes.

## Exercise Compare hash speeds

Been there, done that. Blake3 is a thing of beauty. And the keyed hashing mechanism is just perfection. I love the way the key gets used in each block compression.
The Keccek sponge construction is cool, and I understand why it won at the time,
but I'm confident that if people were reviewing now, Blake2 (the candidate at the time) would have won.
