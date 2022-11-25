//! A placeholder for prototyping some tweakable encryption
//! Calling any of the stuff here will panic through todo!
#[allow(unused_variables)]

/// Blocksize in bytes
pub const BLOCKSIZE_BYTES: usize = 16;

/// Byte to hold a ciphertext or plaintext block
pub struct Block([u8; BLOCKSIZE_BYTES]);

/// The tweak. Must be something unique to the chunk of data being encrypted
/// A disk sector number is a good example.
pub struct Tweak([u8; BLOCKSIZE_BYTES]);

/// Which block within the sector is being encrypted (or decrypted)
pub struct SeqNumber(usize);

/// 32 byte key (256 bits) needed for 128 bit security
pub struct Key256([u8; 32]);

/// 64 byte key (512 bits) needed for 256 bit security
pub struct Key512([u8; 64]);

#[allow(unused_variables)]
/// Encrypts a block of plaintext is XTS-AES mode using AES-128
/// Note that a 256 bit key is needed for 128 bit security.
/// The tweak should be something that uniquely identifies the
/// particlar sequence of blocks being encrypted. A disk sector is
/// a typical case.
/// The block counter is which block we are encrypting within that sector
pub fn encrypt128(key: Key256, plaintext: Block, tweak: Tweak, block_counter: SeqNumber) -> Block {
    unimplemented!()
}

#[allow(unused_variables)]
/// Encrypts a block of plaintext is XTS-AES mode using AES-256
/// Note that a 512 bit key is needed for 256 bit security.
/// The tweak should be something that uniquely identifies the
/// particlar sequence of blocks being encrypted. A disk sector is
/// a typical case.
/// The block counter is which block we are encrypting within that sector
pub fn encrypt256(key: Key512, plaintext: Block, tweak: Tweak, block_counter: SeqNumber) -> Block {
    unimplemented!()
}

