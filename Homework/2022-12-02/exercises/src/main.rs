#[allow(unused_imports)]
use aes::cipher::{
    generic_array::{typenum::U16, GenericArray},
    BlockDecrypt, BlockEncrypt, Key, KeyInit,BlockCipher, BlockEncryptMut,
};

#[allow(unused_imports)]


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

// Not really a union, but a given instance should have
// exactly one of the fields be Some.
#[derive(Debug)]
struct KeyUnion {
    key256: Option<Key<Aes256>>,
    key128: Option<Key<Aes128>>,
    key56: Option<Key<Des>>,

}
#[allow(unused)]
struct Exercise {
    block: Option<GenericArray<u8, U16>>,
    alg: Algorithm,
    key: KeyUnion,
    block_size: usize,
    key_size: usize,
}

#[allow(unused)]
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

        let key = match alg {
            Algorithm::AES256 => {
                let key256 : Key<Aes256> = KeyInit::new_from_slice(&key_vec).expect("can't init key")?;
                KeyUnion {
                    key256: Some(key256),
                    key128: None,
                    key56: None,

                }
            },
            Algorithm::AES128 => {
                let key256 : Key<Aes128> = KeyInit::new_from_slice(&key_vec).expect("can't init key")?;
                KeyUnion {
                    key128: Some(key128),
                    key256: None,
                    key56: None,

                }
            },
            Algorithm::DES => {
                let key56 : Key<Des> = KeyInit::new_from_slice(&key_vec).expect("can't init key")?;
                KeyUnion {
                    key256: None,
                    key128: None,
                    key56: Some(key56),

                }
            },
        };

        // let key = KeyInit::new_from_slice(&key_vec[0..key_size])?;

        let key_slice: &[u8] = &key_vec[0..key_size];
        // let key_slice = &*key_slice;


        let key: &GenericArray<u8, _> = GenericArray::from_slice(key_slice);

        let block: &GenericArray<u8, U16> = GenericArray::from_slice(&block_vec[0..block_size]);

        Ok(Self {
            alg,
            block_size,
            key_size,
            block: Some(*block),
            key: *key,
        })
    }

    fn key(&self) -> GenericArray<u8, U16> {
        self.key
    } 

}

#[allow(unused)]
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

    #[allow(dead_code)]
    let ex3_9 = Exercise::new_with_block(
        Algorithm::AES256,
        key_hex,
        "296C93FDF499AAEB4194BABC2E63561D",
    )
    .unwrap();

    // let cipher = Aes256::new(key);
    // let cipher = Aes256::new(&ex3_9.key());

    let pt_hex = "296C93FDF499AAEB4194BABC2E63561D";

    let pt_slice: &mut [u8; 16] = &mut [0; 16];
    hex::decode_to_slice(pt_hex, pt_slice).expect("failed to de-hexify");
    let pt_slice = &*pt_slice;
    let mut block = GenericArray::from(*pt_slice);

    cipher.encrypt_block(& mut block);
    println!("Ex 3.9\n\t{}", hex::encode(block));
    

}
