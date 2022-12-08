#![allow(dead_code)]

use radix_trie::Trie;
use rand::prelude::*;
use sha2::{Digest, Sha512};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Collision {
    input1: u32,
    input2: u32,
    hash_value: Vec<u8>,
    count: usize,
    length: u16, // length of truncated hash in bits
}

impl fmt::Display for Collision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "msg1: {:08X?}\nmsg2: {:08X?}\nhash: {:02X?}",
            self.input1, self.input2, self.hash_value
        )?;
        write!(f, "\nAfter {} distinct hashes", self.count,)?;
        write!(
            f,
            "\nGoing {:0.3}% of the way through the space",
            self.portion() * 100.0,
        )?;
        write!(
            f,
            "\nWith a {:0.2} probability of getting a collision by this point",
            self.probability(),
        )
    }
}

/// Returns a collision of length-bit hashes or None if none is found
/// Panics if length is not a positive multiple of 8 less than 49
pub(crate) fn hash_collisions(length: u16) -> Option<Collision> {
    if length > 48 {
        panic!("length too long")
    }
    if length % 8 != 0 {
        panic!("length must be a multiple of 8")
    }
    let bytes_to_take = length / 8;

    // I am guessing that creating the hasher once and repeatedly resetting
    // is faster than using digest() each time.
    let mut hasher = Sha512::new();

    // We need a trie to store both the hashes and the messages that led to them
    let mut trie: Trie<Vec<u8>, u32> = radix_trie::Trie::new();

    // should I pick input randomly or just count from 0?
    // I know! I will randomly seed an LCG and draw from that.
    // A properly defined LCG will cycle through all of the values.

    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    let mut acg = AffineGenerator::new(seed);

    let mut count = 0_usize;
    while count <= u32::MAX as usize {
        count += 1;
        let message = acg.next().expect("the ACG cycles and never ends");

        hasher.update(message.to_be_bytes());

        // I feel like there must be a better way of getting my truncated hash
        let hash: Vec<u8> = hasher
            .finalize_reset()
            .iter()
            .take(bytes_to_take.into())
            .copied()
            .collect();

        if let Some(old_message) = trie.insert(hash.to_vec(), message) {
            // we have a collision
            return Some(Collision {
                input1: message,
                input2: old_message,
                hash_value: hash,
                count,
                length,
            });
        }
    }

    None
}

// If I watned to learn generics, I could do this withou fixing it at u32
/// A 32-bit Affine Congruential Generator
/// (frequently called a Linear Congruential Generator)
pub(crate) struct AffineGenerator {
    multiplier: u32,
    increment: u32,
    state: u32,
    modulus_bits: u8,
}

impl AffineGenerator {
    pub(crate) fn new(seed: u32) -> Self {
        // I haven't generalized this for different sizes,
        // so the modulus of 2^32 is hard coded here.
        let modulus_bits = 32_u8;

        // just using fixed, known good values. We'll take the Knuth one
        let multiplier: u32 = 1664525;
        let increment: u32 = 1013904223;

        Self {
            state: seed,
            increment,
            multiplier,
            modulus_bits,
        }
    }
}

impl Iterator for AffineGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);

        Some(self.state)
    }
}

impl Collision {
    /// How far through all possibles hashes the collision was found
    pub fn portion(&self) -> f32 {
        let space_size = 2_u64.pow(self.length as u32) as f32;
        self.count as f32 / space_size
    }

    /// probability of finding a collision at or before this point
    pub fn probability(&self) -> f32 {
        // Birthday parameters are n and d for number
        // of people in the classroom and number of days in the
        // year. I name my variables from that scheme.

        let n = self.count as u32;
        let d = 2_f64.powf(self.length as f64);

        let p = if self.length <= 16 {
            // Small enough for the "exact" formula
            // p = \prod_{i=1}^{n-1}\left(1- \frac{i}{d}\right)
            let mut prod = 1.0;
            for i in 1..n {
                prod *= 1.0 - (i as f64) / d;
            }
            1.0 - prod
        } else {
            // we use the approximate for large _d_
            // p \approx 1 - \exp\left(\frac{n(n-1){2d}\right)
            let n = n as f64;
            let e = -(n * (n - 1.0)) / (2.0 * d);
            1.0 - e.exp()
        };
        p as f32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_8() {
        let c = hash_collisions(8).unwrap();

        println!("{c}");
    }

    #[test]
    fn test_16() {
        let c = hash_collisions(16).unwrap();

        println!("{c}");
    }

    #[test]
    fn test_24() {
        let c = hash_collisions(24).unwrap();

        println!("{c}");
    }

    #[test]
    fn test_32() {
        let c = hash_collisions(32).unwrap();

        println!("{c}");
    }

    #[test]
    #[ignore]
    fn test_48() {
        let c = hash_collisions(48).unwrap();

        println!("{c}");
    }
}
