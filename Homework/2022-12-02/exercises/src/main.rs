#[allow(unused_imports)]
use aes::cipher::{
    generic_array::{typenum::U16, GenericArray},
    BlockDecrypt, BlockEncrypt, Key, KeyInit,BlockCipher, BlockEncryptMut,
};

use hex_literal::hex;


#[allow(unused_imports)]
use aes::{Aes128, Aes256};
#[allow(unused_imports)]
use anyhow::{anyhow, Result, ensure};
#[allow(unused_imports)]
use des::Des;

#[allow(dead_code)]
#[derive(Debug,Clone)]
enum Algorithm {
    AES256,
    AES128,
    DES,
}

// I still don't understand GenericArray, but this helped me get things working
// https://stackoverflow.com/a/60336286/1304076


#[allow(unused)]
fn main() {
    let key = hex!("
        80000000 00000000 00000000 00000000
        00000000 00000000 00000000 00000001");
    let ct = hex!("539B333B39706D149028CFE1D9D4A407");

    let mut block = GenericArray::from(ct);

    let key = GenericArray::from_slice(&key);
    let cipher = Aes256::new(key);

    cipher.decrypt_block(&mut block);
    println!("Ex 3.8\n\t{}", hex::encode(block));


    // exercise 3.9 uses the same key, so I can keep cipher.

    let pt = hex!("296C93FDF499AAEB4194BABC2E63561D");

    // let pt_slice: &mut [u8; 16] = &mut [0; 16];
    // hex::decode_to_slice(pt_hex, pt_slice).expect("failed to de-hexify");
    // let pt_slice = &*pt_slice;
    let mut block = GenericArray::from(pt);

    cipher.encrypt_block(& mut block);
    println!("Ex 3.9\n\t{}", hex::encode(block));
    

}
