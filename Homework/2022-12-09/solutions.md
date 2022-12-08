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
The actual hashes can be stored in a trie,
so that won't be too much of a memory burden.
But if I am going to store the pre-image that resulted in each hash with the hash,
that will space requirement will grow.

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

So my strategy will be to generate 32 bit inputs, and put those inputs as values in a trie that is keyed by the truncated hashes.

I have actually added these computations to my code for talking about a found collision. Here is a sample output with an 8-bit hash

```
msg1: [67, 02, BD, 60]
msg2: [95, 0E, 64, 70]
hash: [79]
After 12 distinct hashes with 0 input collisions
Going 0.05 of the way through the space
With a 0.23 probability of getting a collision by this point
```

and here is a sample with a 32 bit hash.

```
msg1: [D0, 45, 2F, 7F]
msg2: [85, 44, C1, 6A]
hash: [50, DE, 40, AC]
After 105034 distinct hashes with 3 input collisions
Going 0.00 of the way through the space
With a 0.72 probability of getting a collision by this point
```

Because I am picking random 32 bit sequences to hash, there is some possibility that I will pick the same one multiple times. I could increase my input space, but that really eats memory usage.


