use crypto::common::generic_array::GenericArray;
use crypto::common::KeyInit;
use des::cipher::BlockEncrypt;
use des::Des;

/// returns true iff comp(E(key, plaintext)) == E(comp(key), comp(plaintext))
/// Does not parity check key bytes
pub(crate) fn des_comp_check(key: [u8; 8], plaintext: [u8; 8]) -> bool {
    // set up the DES cipher
    let cipher = Des::new(&key.into());

    // compute comp(E(K, pt))
    let mut block: GenericArray<u8, _> = plaintext.into();
    cipher.encrypt_block(&mut block);
    let comp_of_encrypted: [u8; 8] = comp_u8_8(block.as_ref());

    // compute E(comp(K), comp(pt))
    let key_comp = comp_u8_8(&key);
    let cipher_comp = Des::new(&key_comp.into());
    let mut comp_block: GenericArray<u8, _> = comp_u8_8(&plaintext).into();
    cipher_comp.encrypt_block(&mut comp_block);
    let encryption_of_complements = comp_block.as_ref();

    comp_of_encrypted == encryption_of_complements
}

#[inline] // as if the compiler wouldn't have figured to inline this without my suggestion.
fn comp_u8_8(arr: &[u8; 8]) -> [u8; 8] {
    arr.map(|b| !b)
}
