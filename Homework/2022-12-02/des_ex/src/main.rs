use des::cipher::{generic_array::GenericArray, BlockSizeUser, KeyInit, KeySizeUser};
use des::{cipher::BlockEncrypt, Des};
use rand::prelude::*;

type DesKey = GenericArray<u8, <Des as KeySizeUser>::KeySize>;
type DesBlock = GenericArray<u8, <Des as BlockSizeUser>::BlockSize>;

fn des_comp_test(key: [u8; 8], plaintext: [u8; 8]) -> bool {
    // Should we check parity bits on the key? Nah.

    let key_comp: Vec<u8> = key.iter().map(|b| !b).collect::<Vec<u8>>();

    let pt_comp: Vec<u8> = plaintext.iter().map(|b| !b).collect::<Vec<u8>>();

    let key: &DesKey = GenericArray::from_slice(&key);

    let key_comp: &DesKey = GenericArray::from_slice(&key_comp);

    let cipher = Des::new(key);
    let cipher_comp = Des::new(key_comp);

    let mut block: DesBlock = GenericArray::from(plaintext);
    cipher.encrypt_block(&mut block);

    let pt_comp = pt_comp.as_slice();
    let mut comp_block: DesBlock = GenericArray::clone_from_slice(pt_comp);
    cipher_comp.encrypt_block(&mut comp_block);

    let comp1_encrypted_block: Vec<u8> = block.iter().map(|b| !b).collect();

    comp1_encrypted_block == comp_block.to_vec()
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

        if !des_comp_test(rand_key, rand_block) {
            fails += 1;
        }
    }
    println!("{} failure(s) of {} trials", fails, trials);
}
