//! Library for Vigenère cipher.
//!
//! This is just a toy and part of a homework excercise.
//!
//! Also the Vigenère cipher can be broken with paper and pencil with
//! bit of practice ans patience. There is absolutely no reason to use
//! it beyond it being a toy.

#![allow(dead_code)]

use std::collections::HashMap;

pub const DEFAULT_ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Errors happen. We might report them.
pub enum Error {
    /// Something bad happened
    ErrorOps,

    /// Duplicate characters in alphabet
    ErrorDupChars,

    /// Key is too short
    ErrorShortKey,

    /// Key has chars not in alphabet
    ErrorBadKeyChar,

    /// Alphabet is too short
    ErrorShortABC,
}

type AtoIMap = HashMap<char, usize>;
type ItoAMap = HashMap<usize, char>;

/// Alphabet is the ordered set of characters we will cope with.
pub struct Alphabet {
    value: String,
    length: usize,
    atoi_map: HashMap<char, usize>,
    itoa_map: HashMap<usize, char>,
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl Alphabet {
    // builds the position/character associative arrays
    // also validates that the input alphabet doesn't have duplicates
    fn build_maps(alphabet: &str) -> Result<(AtoIMap, ItoAMap), Error> {
        let mut atoi: AtoIMap = HashMap::new();
        let mut itoa: ItoAMap = HashMap::new();

        for (i, c) in alphabet.chars().enumerate() {
            if atoi.contains_key(&c) {
                return Err(Error::ErrorDupChars);
            }
            atoi.insert(c, i);
            itoa.insert(i, c);
        }
        Ok((atoi, itoa))
    }

    /// checks that every char in key is in alphabet
    pub fn is_key_valid(&self, key: &str) -> bool {
        for c in key.chars() {
            if !self.atoi_map.contains_key(&c) {
                return false;
            }
        }
        true
    }
}

impl Alphabet {
    /// Creates an Alphabet using the [DEFAULT_ABC]
    pub fn new() -> Self {
        match Self::try_from(DEFAULT_ABC.to_string()) {
            Err(_) => panic!("default should not error"),
            Ok(abc) => abc,
        }
    }

    // We do input validation when constructing objects,
    // so safe to panic here if things are malformed.
    fn add(&self, a: char, b: char) -> char {
        let a_pos = self.atoi_map.get(&a).unwrap();
        let b_pos = self.atoi_map.get(&b).unwrap();
        let new_pos = (a_pos + b_pos) % self.length;
        let c = self.itoa_map.get(&new_pos).unwrap();
        *c
    }
    #[inline]
    fn sub(&self, a: char, b: char) -> char {
        self.add(a, self.inv(b))
    }
    #[inline]
    // return the additive inverse of a character given the alphabet
    fn inv(&self, c: char) -> char {
        let pos = self.atoi_map.get(&c).unwrap();
        let neg_pos = self.length - pos;
        let inv_c = self.itoa_map.get(&neg_pos).unwrap();
        *inv_c
    }
}

impl TryFrom<String> for Alphabet {
    type Error = Error;
    /// Will Error if s contains duplicate characters or if s is shorter than 2
    fn try_from(s: String) -> Result<Self, Error> {
        if s.len() < 2 {
            return Err(Error::ErrorShortABC);
        }
        let value = s.clone();
        let length = s.len();
        let (atoi_map, itoa_map) = Self::build_maps(&s)?;
        Ok(Self {
            value,
            length,
            atoi_map,
            itoa_map,
        })
    }
}

/// Vigenere is where we will put our key and alphabet
pub struct Vigenere {
    /// The alphabet is an ordered sequence of unique characters.
    alphabet: Alphabet,

    // The key must be a string of characters all within the alphabet
    key: String,
}

impl Vigenere {
    fn valid_key_or_err(key: &str, alphabet: &Alphabet) -> Result<(), Error> {
        if key.is_empty() {
            return Err(Error::ErrorShortKey)
        }
        if !alphabet.is_key_valid(key) {
            return Err(Error::ErrorBadKeyChar)
        }
        Ok(())
    }

    pub fn new(key: &str) -> Result<Self, Error> {
        let alphabet: Alphabet = Alphabet::new();
        Self::valid_key_or_err(key, &alphabet)?;
        Ok(Self {
            alphabet,
            key: key.into(),
        })
    }

    pub fn new_with_alphabet(key: &str, alphabet: String) -> Result<Self, Error> {
        let alphabet: Alphabet = Alphabet::try_from(alphabet)?;
        Self::valid_key_or_err(key, &alphabet)?;
        Ok(Self {
            key: key.to_string(),
            alphabet,
        })
    }
}
