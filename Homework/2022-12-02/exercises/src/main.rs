use std::iter::zip;

use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

// type AesBlock256 = GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>;
use aes::Aes256;

use hex_literal::hex;

// additional for 3.10
mod des_comp;
use crate::des_comp::des_comp_check;
use rand::prelude::*;

mod padding;
use crate::padding::*;

// additional imports for 4.4
use aes::cipher::{BlockDecryptMut, KeyIvInit};

// type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

fn ex_3_8_9() -> String {
    let key = hex!(
        "
        80000000 00000000 00000000 00000000
        00000000 00000000 00000000 00000001"
    );
    let ct = hex!("539B333B39706D149028CFE1D9D4A407");

    let cipher = Aes256::new(&key.into());

    let mut block = ct.into();
    cipher.decrypt_block(&mut block);
    let ex38 =  format!("Ex 3.8\n\t{}", hex::encode(block));

    // exercise 3.9 uses the same key, so I can keep cipher.
    let pt = hex!("296C93FDF499AAEB4194BABC2E63561D");
    let mut block = pt.into();

    cipher.encrypt_block(&mut block);

    format!("{}\nEx 3.9\n\t{}", ex38, hex::encode(block))
}

fn ex_3_10() -> String {
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
    format!("Ex 3.10:\n\t{} failure(s) of {} trials", fails, trials)
}

#[allow(unused)]
fn ex_4_3() -> String {
    let c1 = hex!(
        "
                    46 64 DC 06 97 BB FE 69
                    33 07 15 07 9B A6 C2 3D
                    2B 84 DE 4F 90 8D 7D 34
                    AA CE 96 8B 64 F3 DF 75
                "
    );
    let c2 = hex!(
        "
                    51 7E CC 05 C3 BD EA 3B
                    33 57 0E 1B D8 97 D5 30
                    7B D0 91 6B 8D 82 6B 35
                    B7 8B BB 8D 74 E2 C7 3B
                "
    );
    let p1 = hex!(
        "
                    43 72 79 70 74 6F 67 72
                    61 70 68 79 20 43 72 79
                    70 74 6F 67 72 61 70 68
                    79 20 43 72 79 70 74 6F
    "
    );

    // pad = c1 xor p1
    let pad: Vec<u8> = zip(c1, p1).map(|(b1, b2)| b1 ^ b2).collect();

    // pt2 = pad xor c2
    let p2: Vec<u8> = zip(pad, c2).map(|(b1, b2)| b1 ^ b2).collect();

    // println!("Ex 4.3: P'\n\t{:X?}", p2);
    // We get p2 [84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 115, 101, 99, 114, 101, 116, 32, 32, 32, 67, 111, 110, 102, 105, 100, 101, 110, 116, 105, 97, 108, 33]
    // That really looks like ASCII printable range to me.

    let p2_text: String = p2.iter().map(|c| *c as char).collect();
    format!("Ex 4.3: Pʹ\n\t{}", p2_text)
}

fn ex_4_4()  -> String {
    let iv = hex!("87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91");
    let ct = hex!(
        "7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92
                             6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17"
    );
    let key = hex!(
        "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                              00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01"
    );

    // buf needs to be long enough for the plaintext + padding block

    let ct_b0: [u8; 16] = ct[0..16].try_into().expect("16 should equal 16");
    let ct_b1: [u8; 16] = ct[16..32].try_into().expect("16 should equal 16");

    let block0 = ct_b0.into();
    let block1 = ct_b1.into();

    let mut cipher = Aes256CbcDec::new(&key.into(), &iv.into());
    cipher.decrypt_blocks_mut(&mut [block0, block1]);

    format!("Ex 4.4: plaintext\n\t{:02X?}\n\t{:02X?}", block0, block1)
}

// The task is to implement a scheme. It doesn't ask to compute anything.
fn ex_4_6() -> String {
    let unpadded5 = hex!("FF FF FF FF FF");
    let padded5 = hex!(
        "FF FF FF FF FF 0B 0B 0B
            0B 0B 0B 0B 0B 0B 0B 0B"
    );

    let result = pkcs_padder(&unpadded5);
    assert_eq!(result, padded5);

    let unpadded16 = [0u8; 16];
    let mut padded16 = unpadded16.to_vec();
    padded16.append(&mut vec![16u8; 16]);

    let r16 = pkcs_padder(&unpadded16);

    let mut test_count = 0;
    let mut test_failures = 0;

    test_count += 1;
    if r16 != padded16 {
        test_failures += 1;
    }

    struct TestVector {
        _name: String,
        input: Vec<u8>,
        expected: bool,
    }

    let tests = vec![
        TestVector {
            _name: "Short".into(),
            input: vec![1, 2, 3, 4],
            expected: false,
        },
        TestVector {
            _name: "Long".into(),
            input: vec![1u8; 17],
            expected: false,
        },
        TestVector {
            _name: "four pad".into(),
            input: vec![4; 16],
            expected: true,
        },
    ];

    for t in tests {
        test_count += 1;
        let r = pkcs_validator(t.input);
        if r != t.expected {
            test_failures += 1;
        }
    }

    format!(
        "Ex 4.6\n\t{} failures of {} padding/verifying tests",
        test_failures, test_count
    )
}

fn main() {
    let exercise_funcs = vec![
                        ex_3_8_9 as fn() -> String,
                        ex_3_10,
                        ex_4_3,
                        ex_4_4,
                        ex_4_6,
                    ];

    for f in exercise_funcs {
        println!("{}", f());
    }
}
