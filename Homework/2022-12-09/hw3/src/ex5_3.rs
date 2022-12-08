mod ex5_3 {
    use sha2::{Digest, Sha512};

    use radix_trie::Trie;
    use rand::prelude::*;

    #[allow(dead_code)]
    #[derive(Debug,Clone)]
    pub(crate) struct Collision {
        input1: Vec<u8>,
        input2: Vec<u8>,
        hash_value: Vec<u8>,
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

        loop {
            rng.fill_bytes(&mut message);
            hasher.update(message);

            // I feel like there must be a better way of getting my truncated hash
            let hash: Vec<u8> = hasher.finalize_reset()
                .iter()
                .take(bytes_to_take)
                .map(|b| *b)
                .collect();

            if let Some(old_message) = trie.insert(hash.to_vec(), message) {
                if old_message != message {
                    // we have a collision
                    return Collision {
                        input1: message.to_vec(),
                        input2: old_message.to_vec(),
                        hash_value: hash,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ex5_3::hash_collisions;

    #[test]
    fn test_8() {
        let c = hash_collisions(8);
        
        println!("{c:?}");
    }

    #[test]
    fn test_16() {
        let c = hash_collisions(16);
        
        println!("{c:?}");
    }

    #[test]
    fn test_24() {
        let c = hash_collisions(24);
        
        println!("{c:?}");
    }

    #[test]
    fn test_32() {
        let c = hash_collisions(32);
        
        println!("{c:?}");
    }

    #[test]
    fn test_48() {
        let c = hash_collisions(48);
        
        println!("{c:?}");
    }
}

