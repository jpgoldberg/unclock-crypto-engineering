#[allow(unused_imports)]


use crypto_common::{KeyInit, KeySizeUser, BlockSizeUser, Block,generic_array::GenericArray,};

use aes::{Aes128, Aes256};

use hex_literal::hex;


#[allow(unused_imports)]
use anyhow::{anyhow, Result, ensure};

#[allow(dead_code)]
#[derive(Debug,Clone)]
enum Algorithm {
    AES256,
    AES128,
    DES,
}

// I still don't understand GenericArray, but this helped me get things working
// https://stackoverflow.com/a/60336286/1304076

type AesKey128 = GenericArray<u8, <Aes128 as KeySizeUser>::KeySize>;
type AesKey256 = GenericArray<u8, <Aes256 as KeySizeUser>::KeySize>;
type AesBlock256 = GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>;


#[allow(unused)]
fn main() {
    let key = hex!("
        80000000 00000000 00000000 00000000
        00000000 00000000 00000000 00000001");
    let ct = hex!("539B333B39706D149028CFE1D9D4A407");

    let mut block = GenericArray::from(ct);

    let key = GenericArray::from_slice(&key);
    let cipher = Aes256::new(key);

    aes::cipher::BlockDecrypt::decrypt_block(&cipher, &mut block);
    println!("Ex 3.8\n\t{}", hex::encode(block));


    // exercise 3.9 uses the same key, so I can keep cipher.

    let pt = hex!("296C93FDF499AAEB4194BABC2E63561D");
    let mut block = GenericArray::from(pt);

    aes::cipher::BlockEncrypt::encrypt_block(&cipher, & mut block);
    println!("Ex 3.9\n\t{}", hex::encode(block));
    

}
