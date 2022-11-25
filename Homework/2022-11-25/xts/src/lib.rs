//! A placeholder for prototyping some
//! XTS-AES ([IEEE-XTS], [NIST-XTS]) tweakable encryption.
//! 
//! None of the functions are implemented.
//! 
//! [NIST-XTS]: https://csrc.nist.gov/publications/detail/sp/800-38e/final "NIST Recommendation for Block Cipher Modes of Operation: The XTS-AES Mode for Confidentiality on Storage Devices"
//! [IEEE-XTS]: https://standards.ieee.org/ieee/1619/4205/ "IEEE IEEE Standard for Cryptographic Protection of Data on Block-Oriented Storage Devices (paywalled)"

/// Blocksize in bytes
pub const BLOCKSIZE_BYTES: usize = 16;

/// Maximum number of blocks per Tweak from [NIST-XTS]
pub const MAX_SEQ_NUMBER: u32 = 1_048_576;

/// Byte to hold a ciphertext or plaintext block
pub struct Block([u8; BLOCKSIZE_BYTES]);

/// The tweak. Must be something unique to the chunk of data being encrypted
/// A disk sector number is a good example.
pub struct Tweak([u8; BLOCKSIZE_BYTES]);

/// Which block within the tweak domain
/// (e.g, within a disk sector)
/// is being encrypted (or decrypted)
/// 
/// [IEEE-XTS] says that this _should_ not exceed 2^20.
/// [NIST-XTS] turns that "should not" into a "shall not".
pub struct SeqNumber(u32);

/// 32 byte key (256 bits) needed for 128 bit security
pub struct Key256([u8; 32]);

/// 64 byte key (512 bits) needed for 256 bit security
pub struct Key512([u8; 64]);

#[allow(unused_variables)]
/// Encrypts a block of plaintext in XTS-AES mode using AES-128
/// 
/// key: A 256 bit key is needed for 128 bit security
/// 
/// plaintext: A [Block] of bytes (u8) to be encrypted
/// 
/// tweak: An identifier for set of blocks the plaintext is in.
/// A disk sector number is a good choice.
/// 
/// block_counter: Which block we are encrypting within that sector.
/// panics if block_counter exceeds [MAX_SEQ_NUMBER]
pub fn encrypt128(key: Key256, plaintext: Block, tweak: Tweak, block_counter: SeqNumber) -> Block {
    if block_counter.0 > MAX_SEQ_NUMBER {
        panic!("Too many blocks in a tweak domain")
    }
    unimplemented!()
}

#[allow(unused_variables)]
/// Encrypts a block of plaintext in XTS-AES mode using AES-256
///
/// parameters the same as for [encrypt128] except
/// the key is 64 bytes (512 bits) to achive 256 bit security.
pub fn encrypt256(key: Key512, plaintext: Block, tweak: Tweak, block_counter: SeqNumber) -> Block {
    if block_counter.0 > MAX_SEQ_NUMBER {
        panic!("Too many blocks in a tweak domain")
    }
    unimplemented!()
}

