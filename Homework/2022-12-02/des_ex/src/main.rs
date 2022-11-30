#[allow(unused)]
use des::cipher::{generic_array::GenericArray, Block, BlockSizeUser, KeyInit, KeySizeUser};
use des::Des;

use anyhow::Result;

type DesKey = GenericArray<u8, <Des as KeySizeUser>::KeySize>;
type DesBlock = GenericArray<u8, <Des as BlockSizeUser>::BlockSize>;


fn des_comp_test(key: [u8; 56], plaintext: &[u8; 64]) -> Result<bool> {

    let key_comp: Vec<u8> = key
        .iter()
        .map(|b| !b)
        .collect::<Vec<u8>>();

    let pt_comp: Vec<u8> = plaintext
        .iter()
        .map(|b| !b)
        .collect::<Vec<u8>>();

    let key: &DesKey = GenericArray::from_slice(&key);

    let key_comp: &DesKey = GenericArray::from_slice(&key_comp);
    
    let cipher = Des::new(key);
    let cipher_comp = Des::new(key_comp);


    Ok(true)
}
fn main() {

}
