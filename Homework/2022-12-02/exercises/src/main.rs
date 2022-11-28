#[allow(unused_imports)]
use aes::cipher::{
    generic_array::{typenum::U16, GenericArray},
    BlockDecrypt, Key, KeyInit,BlockCipher, BlockEncryptMut,
};

#[allow(unused_imports)]


#[allow(unused_imports)]
use aes::{Aes128, Aes256};
#[allow(unused_imports)]
use anyhow::{anyhow, Result, ensure};
#[allow(unused_imports)]
use des::Des;

enum Algorithm {
    AES256,
    AES128,
    DES,
}

// I still don't understand GenericArray, but this helped me get things working
// https://stackoverflow.com/a/60336286/1304076
struct Exercise {
    block: Option<GenericArray<u8, U16>>,
    alg: Algorithm,
    key: GenericArray<u8, U16>,
    block_size: usize,
    key_size: usize,
}

impl Exercise {
    fn new_with_block(alg: Algorithm, key_hex: &str, block_hex: &str) -> Result<Self> {
        let (block_size, key_size) = match alg {
            Algorithm::AES256 => (16_usize, 32_usize),
            Algorithm::AES128 => (16, 16),
            Algorithm::DES => (8, 7),
        };

        let block_vec = hex::decode(block_hex)?;
        if block_vec.len() != block_size {
            return Err(anyhow!("bad size of block"));
        }
        let key_vec = hex::decode(key_hex)?;
        if key_vec.len() != key_size {
            return Err(anyhow!("key is wrong length"));
        }

        let block: &GenericArray<u8, U16> = GenericArray::from_slice(&block_vec[0..block_size]);

        // let key = KeyInit::new_from_slice(&key_vec[0..key_size])?;

        let key_slice = &key_vec[0..key_size];

        let key = GenericArray::from_slice(key_slice);

        Ok(Self {
            alg,
            block_size,
            key_size,
            block: Some(*block),
            key: *key,
        })
    }

    fn key(&self) -> GenericArray<u8, U16> {
        self.key.clone()
    } 

}

fn main() {
    let ct_hex = "539B333B39706D149028CFE1D9D4A407";
    let key_hex = "8000000000000000000000000000000000000000000000000000000000000001";

    let ct_slice: &mut [u8; 16] = &mut [0; 16];
    hex::decode_to_slice(ct_hex, ct_slice).expect("I expected better of this");
    let ct_slice = &*ct_slice;

    let mut block = GenericArray::from(*ct_slice);

    let key_slice: &mut [u8; 32] = &mut [0; 32];
    hex::decode_to_slice(key_hex, key_slice).expect("failed to hex decode key");
    let key = GenericArray::from_slice(&*key_slice);

    let cipher = Aes256::new(key);
    cipher.decrypt_block(&mut block);
    println!("Ex 3.8\n\t{}", hex::encode(block));

    // exercise 3.9 uses the same key, so I can keep cipher.

    let ex3_9 = Exercise::new_with_block(
        Algorithm::AES256,
        key_hex,
        "296C93FDF499AAEB4194BABC2E63561D",
    )
    .unwrap();

    // let cipher = Aes256::new(key);
    let cipher = Aes256::new(&ex3_9.key());
    

}
