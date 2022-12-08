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

Let's see ahead of time. 48 bits is six bytes.
In the worst case, we need to store nearly $2^{48}$ truncated hashes, but it really shouldn't be much more than $2^{24}$. I am going to limit things to $2^{32}$ so that I can store
inputs in a `u32`.
It is a near certainty to have a collision at that point.

For reasons I can no longer recall, I have a birthday collision approximation calculator
in R.

```R
pbirthday <- function(n, d) {
    exponent <- - (n * (n - 1))/( 2 * d)
    p <- 1 - exp(exponent)
    return(p);
}
```

And a run of it confirms that $2^{32}$ inputs is more than sufficient.

```Rconsole
> source("/Users/jeffrey/Dropbox/AWS/AGConf7/birthday.R", encoding = "UTF-8")
> pbirthday(2^32, 2^48)
[1] 1
```

So my strategy will be to generate 32 bit inputs, and put those inputs as values in a trie that is keyed by the truncated hashes.


