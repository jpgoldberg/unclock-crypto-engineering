use crypto_common::BlockSizeUser;
use crypto_common::generic_array::GenericArray;
use des::cipher::KeyInit;
use des::{cipher::BlockEncrypt, Des};
use rand::prelude::*;

type DesBlock = GenericArray<u8, <Des as BlockSizeUser>::BlockSize>;

/// returns true iff comp(E(key, plaintext)) == E(comp(key), comp(plaintext))
/// Does not parity check key bytes
fn des_comp_check(key: [u8; 8], plaintext: [u8; 8]) -> bool {
    
    // compute comp(E(K, pt))
    let cipher = Des::new(&key.into());
    let mut block: DesBlock = plaintext.into();
    cipher.encrypt_block(&mut block);
    let comp_of_encrypted: [u8; 8] = comp_u8_8(block.as_ref());

    // compute E(comp(K), comp(pt))
    let key_comp = comp_u8_8(&key);
    let cipher_comp = Des::new(&key_comp.into());
    let mut comp_block: DesBlock = comp_u8_8(&plaintext).into();
    cipher_comp.encrypt_block(&mut comp_block);
    let encryption_of_complements = comp_block.as_ref();

    comp_of_encrypted == encryption_of_complements
}

#[inline] // as if the compiler wouldn't have figured to inline this without my suggestion.
fn comp_u8_8(arr: &[u8; 8]) -> [u8; 8] {
    arr.map(|b| !b)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut rand_key = [0u8; 8];
    let mut rand_block = [0u8; 8];

    let trials: u32 = 100;
    let mut fails: u32 = 0;
    for _ in 1..=trials {
        rng.fill_bytes(&mut rand_key);
        rng.fill_bytes(&mut rand_block);

        if !des_comp_check(rand_key, rand_block) {
            fails += 1;
        }
    }
    println!("{} failure(s) of {} trials", fails, trials);
}
