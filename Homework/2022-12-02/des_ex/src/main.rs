use des::cipher::KeyInit;
use des::{cipher::BlockEncrypt, Des};
use rand::prelude::*;

/// returns true iff comp(E(key, plaintext)) == E(comp(key), comp(plaintext))
/// Does not parity check key bytes
fn des_comp_check(key: [u8; 8], plaintext: [u8; 8]) -> bool {
    
    // compute comp(E(K, pt))
    let cipher = Des::new(&key.into());
    let block = plaintext;
    cipher.encrypt_block(&mut block.into());
    let comp_of_encrypted = comp_u8_8(&block);

    // compute E(comp(K), comp(pt))
    let key_comp = comp_u8_8(&key);
    let cipher_comp = Des::new(&key_comp.into());
    let comp_block = comp_u8_8(&plaintext);
    cipher_comp.encrypt_block(&mut comp_block.into());
    let encryption_of_complements = comp_block;

    comp_of_encrypted == encryption_of_complements
}

fn comp_u8_8(arr: &[u8; 8]) -> [u8; 8] {
    arr.iter()
        .map(|b| !b)
        .collect::<Vec<u8>>()
        .as_slice()
        .try_into()
        .expect("uh oh. 8 should equal 8")
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
