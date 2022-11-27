use aes::Aes256;
use aes::cipher::{KeyInit, generic_array::GenericArray, BlockDecrypt};

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

}
