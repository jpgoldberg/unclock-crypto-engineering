use crate::ex5_3::AffineGenerator;
use rand::Rng;
use sha2::{Digest, Sha512};
// use blake3::Hasher;

#[allow(dead_code)]
pub fn find_preimage_for_prefix(prefix: Vec<u8>) -> u32 {
    if prefix.len() > 3 {
        panic!("Nope. I won't try that!")
    }

    let mut hasher = Sha512::new();

    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    let mut acg = AffineGenerator::new(seed);

    loop {
        let m = acg.next().expect("Will this ever end?");
        hasher.update(m.to_be_bytes());

        let hash: Vec<u8> = hasher.finalize_reset().to_vec();

        if hash.starts_with(&prefix) {
            return m;
        }
    }
}

pub fn b3_find_preimage_for_prefix(prefix: Vec<u8>) -> u32 {
    if prefix.len() > 3 {
        panic!("Nope. I won't try that!")
    }

    let mut hasher = blake3::Hasher::new();

    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    let mut acg = AffineGenerator::new(seed);

    loop {
        let m = acg.next().expect("Will this ever end?");
        hasher.update(&m.to_be_bytes());

        let hash: Vec<u8> = hasher.finalize_reset().to_vec();

        if hash.starts_with(&prefix) {
            return m;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_ex5_4() {
        let prefix = hex!("3D 4B");

        let pre_image = find_preimage_for_prefix(prefix.into());

        println!("{}", pre_image);
    }

    #[test]
    fn test_b3_ex5_4() {
        let prefix = hex!("3D 4B");

        let pre_image = b3_find_preimage_for_prefix(prefix.into());

        println!("{}", pre_image);
    }

    #[test]
    #[ignore]
    fn test_3_bytes() {
        let prefix = [1_u8, 2, 3];

        let pre_image = find_preimage_for_prefix(prefix.into());

        println!("{}", pre_image);
    }
}
