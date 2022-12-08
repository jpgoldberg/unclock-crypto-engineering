use radix_trie::Trie;
use rand::prelude::*;
use sha2::{Digest, Sha512};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Collision {
    input1: Vec<u8>,
    input2: Vec<u8>,
    hash_value: Vec<u8>,
    count: usize,
    input_collisions: usize,
    length: usize, // length of truncated hash in bits
}

impl fmt::Display for Collision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "msg1: {:02X?}\nmsg2: {:02X?}\nhash: {:02X?}",
            self.input1, self.input2, self.hash_value
        )?;
        write!(
            f,
            "\nAfter {} distinct hashes with {} input collisions",
            self.count, self.input_collisions,
        )?;
        write!(
            f,
            "\nGoing {:0.2} of the way through the space",
            self.portion(),
        )?;
        write!(
            f,
            "\nWith a {:0.2} probability of getting a collision by this point",
            self.probability(),
        )
    }
}

#[allow(dead_code)]
/// Returns a collision of length-bit hashes or None if none is found
/// Panics if length is not a positive multiple of 8 less than 49
pub(crate) fn hash_collisions(length: usize) -> Collision {
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
    let mut trie: Trie<Vec<u8>, [u8; 4]> = radix_trie::Trie::new();

    // should I pick input randomly or just count from 0?
    let mut rng = rand::thread_rng();
    let mut message = [0u8; 4];

    let mut count = 32_usize;
    let mut input_collisions = 0_usize;
    loop {
        count += 1;
        rng.fill_bytes(&mut message);
        hasher.update(message);

        // I feel like there must be a better way of getting my truncated hash
        let hash: Vec<u8> = hasher
            .finalize_reset()
            .iter()
            .take(bytes_to_take)
            .copied()
            .collect();

        if let Some(old_message) = trie.insert(hash.to_vec(), message) {
            if old_message != message {
                // we have a collision
                return Collision {
                    input1: message.to_vec(),
                    input2: old_message.to_vec(),
                    hash_value: hash,
                    count: count - input_collisions,
                    input_collisions,
                    length,
                };
            } else {
                input_collisions += 1;
            };
        }
    }
}

impl Collision {
    /// How far through all possibles hashes the collision was found
    pub fn portion(&self) -> f32 {
        let space_size = 2_u64.pow(self.length as u32) as f32;
        self.count as f32 / space_size
    }

    /// probability of finding a collision at or before this point
    pub fn probability(&self) -> f64 {
        // Birthday parameters are n and d for number
        // of people in the classroom and number of days in the
        // year. I name my variable from that scheme.

        // TODO: I should do exact calculation if length is small enough

        // The exact formula is \prod_{i=1}^n\left(1- \frac{i}{d}\right)
        // So it requires one floating point multiplication and division for
        // each d.

        let n = self.count as u32;
        let d = 2_f64.powf(self.length as f64);

        let p = match self.length <= 16 {
            true => {
                let mut prod = 1.0;
                for i in 1..n {
                    prod *= 1.0 - (i as f64) / d;
                }
                1.0 - prod
            }
            false => {
                let n = n as f64;
                let e = -(n * (n - 1.0)) / (2.0 * d);
                1.0 - e.exp()
            }
        };

        // Hey clippy. I am explicitly creating a temporary variable
        // so that I can see it when debugging. The compiler will
        // optimize away my verbosity.
        p
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_8() {
        let c = hash_collisions(8);

        println!("{c}");
    }

    #[test]
    fn test_16() {
        let c = hash_collisions(16);

        println!("{c}");
    }

    #[test]
    fn test_24() {
        let c = hash_collisions(24);

        println!("{c}");
    }

    #[test]
    fn test_32() {
        let c = hash_collisions(32);

        println!("{c}");
    }

    #[test]
    fn test_48() {
        let c = hash_collisions(48);

        println!("{c}");
    }
}
