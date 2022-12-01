use std::iter::zip;

use aes::cipher::{
    generic_array::GenericArray, BlockDecrypt, BlockEncrypt, BlockSizeUser, KeyInit,
};
use aes::Aes256;
use hex_literal::hex;

type AesBlock256 = GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>;

fn ex_3_8_9() {
    let key = hex!(
        "
        80000000 00000000 00000000 00000000
        00000000 00000000 00000000 00000001"
    );
    let ct = hex!("539B333B39706D149028CFE1D9D4A407");

    let cipher = Aes256::new(&key.into());

    let mut block: AesBlock256 = ct.into();
    cipher.decrypt_block(&mut block);
    println!("Ex 3.8\n\t{}", hex::encode(block));

    // exercise 3.9 uses the same key, so I can keep cipher.
    let pt = hex!("296C93FDF499AAEB4194BABC2E63561D");
    let mut block: AesBlock256 = pt.into();

    cipher.encrypt_block(&mut block);
    println!("Ex 3.9\n\t{}", hex::encode(block));
}

#[allow(unused)]
fn ex_4_3() {
    let c1 = hex!("
                    46 64 DC 06 97 BB FE 69
                    33 07 15 07 9B A6 C2 3D
                    2B 84 DE 4F 90 8D 7D 34
                    AA CE 96 8B 64 F3 DF 75
                ");
    let c2 = hex!("
                    51 7E CC 05 C3 BD EA 3B
                    33 57 0E 1B D8 97 D5 30
                    7B D0 91 6B 8D 82 6B 35
                    B7 8B BB 8D 74 E2 C7 3B
                ");
    let p1 = hex!("
                    43 72 79 70 74 6F 67 72
                    61 70 68 79 20 43 72 79
                    70 74 6F 67 72 61 70 68
                    79 20 43 72 79 70 74 6F
    ");

    // pad = c1 xor p1
    let pad: Vec<u8> = zip(c1, p1).map(|(b1, b2)| b1 ^ b2).collect();

    // pt2 = pad xor c2
    let p2: Vec<u8> = zip(pad.iter(), c2).map(|(b1, b2)| b1 ^ b2).collect();

    // println!("Ex 4.3: Pʹ\n\t{}", p2_text);
    // We get p2 [84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 115, 101, 99, 114, 101, 116, 32, 32, 32, 67, 111, 110, 102, 105, 100, 101, 110, 116, 105, 97, 108, 33]
    // That really looks like ASCII printable range to me.

    let p2_text: String = p2.iter().map(|c| *c as char).collect();
    
    println!("Ex 4.3: Pʹ\n\t{:?}", p2);


}

fn main() {
    ex_3_8_9();
    ex_4_3()
}
