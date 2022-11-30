use des::cipher::KeyInit;
use des::{cipher::BlockEncrypt, Des};
use rand::prelude::*;

/// returns true iff comp(E(key, plaintext)) == E(comp(key), comp(plaintext))
fn des_comp_check(key: [u8; 8], plaintext: [u8; 8]) -> bool {
    // Should we check parity bits on the key? Nah.

    let key_comp: [u8; 8] = key
        .iter()
        .map(|b| !b)
        .collect::<Vec<u8>>()
        .as_slice()
        .try_into()
        .expect("uh oh");

    let pt_comp: [u8; 8] = plaintext
        .iter()
        .map(|b| !b)
        .collect::<Vec<u8>>()
        .as_slice()
        .try_into()
        .expect("shouldn't happen");

    let cipher = Des::new(&key.into());

    let cipher_comp = Des::new(&key_comp.into());

    let block = plaintext;
    cipher.encrypt_block(&mut block.into());
    let comp_of_encrypted: Vec<u8> = block.iter().map(|b| !b).collect();

    cipher_comp.encrypt_block(&mut pt_comp.into());
    let encryption_of_complements = pt_comp;

    comp_of_encrypted == encryption_of_complements.to_vec()
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
