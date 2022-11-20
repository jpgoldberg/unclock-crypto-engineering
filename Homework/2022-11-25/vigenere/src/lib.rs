//! Library for Vigenère cipher.
//!
//! This is just a toy and part of a homework excercise.
//!
//! Also the Vigenère cipher can be broken with paper and pencil with
//! bit of practice ans patience. There is absolutely no reason to use
//! it beyond it being a toy.

#![allow(dead_code)]

use std::collections::HashMap;

/// Errors happen. We might report them.
pub enum Error {
    /// Something bad happened
    ErrorOps,
}

type AtoIMap = HashMap<char, usize>;
type ItoAMap = HashMap<usize, char>;

/// Alphabet is the ordered set of characters we will cope with
pub struct Alphabet {
    value: String,
    length: usize,
    atoi_map: HashMap<char, usize>,
    itoa_map: HashMap<usize, char>,
}

impl Alphabet {
    fn build_maps(alphabet: &str) -> (AtoIMap, ItoAMap) {
        let mut atoi: AtoIMap = HashMap::new();
        let mut itoa: ItoAMap = HashMap::new();

        for (i, c) in alphabet.chars().enumerate() {
            atoi.insert(c, i);
            itoa.insert(i, c);
        }
        (atoi, itoa)
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        let value: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let length: usize = 26;
        let (atoi_map, itoa_map) = Self::build_maps(&value);

        Self {
            value,
            length,
            atoi_map,
            itoa_map,
        }
    }
}

impl Alphabet {
    fn new() -> Self {
        Default::default()
    }

    fn add(&self, a: char, b: char) -> Option<char> {
        let a_pos = self.atoi_map.get(&a)?;
        let b_pos = self.atoi_map.get(&b)?;
        let new_pos = (a_pos + b_pos) % self.length;
        let c = self.itoa_map.get(&new_pos)?;
        Some(*c)
    }
    #[inline]
    fn neg(&self, pos: usize) -> usize {
        self.length - pos
    }
    fn sub(&self, a: char, b: char) -> Option<char> {
        let a_pos = self.atoi_map.get(&a)?;
        let b_pos = self.atoi_map.get(&b)?;
        let new_pos = (a_pos + self.neg(*b_pos)) % self.length;
        let c = self.itoa_map.get(&new_pos)?;
        Some(*c)
    }
}

impl TryFrom<String> for Alphabet {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Error> {
        // validiate_alphabet(s)?
        let value = s.clone();
        let length = s.len();
        let (atoi_map, itoa_map) = Self::build_maps(&s);
        Ok(Self {value, length, atoi_map, itoa_map})
    }
}

/// Vigenere is where we will put our key and alphabet
pub struct Vigenere {
    /// The alphabet is an ordered sequence of unique characters.
    pub alphabet: Alphabet,

    // The key must be a string of characters all within the alphabet
    key: String,
}

impl Vigenere {
    pub fn new(key: &str) -> Result<Self, Error> {
        let alphabet: Alphabet = Default::default();
        Ok(Self {
            alphabet,
            key: key.into(),
        })
    }

    pub fn new_with_alphabet(key: &str, alphabet: String) -> Result<Self, Error> {
        let key = key.to_string();
        // validate_key(key)?
        let alphabet: Alphabet = Alphabet::try_from(alphabet)?;
        Ok(Self { key, alphabet })
    }
}
