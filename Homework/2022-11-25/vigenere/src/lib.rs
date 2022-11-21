//! Library for Vigenère cipher.
//!
//! This is just a toy and part of a homework excercise.
//!
//! Also the Vigenère cipher can be broken with paper and pencil with
//! bit of practice ans patience. There is absolutely no reason to use
//! it beyond it being a toy.

#![allow(dead_code)]

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

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

pub struct Key {
    value: Vec<String>,
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}
impl From<String> for Key {
    fn from(s: String) -> Self {
        let value: Vec<String> = s
            .to_uppercase()
            .graphemes(true)
            .map(|s| s.to_string())
            .collect();
        Self { value }
    }
}

impl Key {
    fn len(&self) -> usize {
        self.value.len()
    }

    fn char_from_position(&self, pos: usize) -> String {
        let pos = pos % self.len();
        self.value[pos].clone()
    }
}

type AtoIMap = HashMap<String, usize>;
type ItoAMap = HashMap<usize, String>;

/// Alphabet is the ordered set of characters we will cope with.
pub struct Alphabet {
    value: Vec<String>,
    length: usize,
    atoi_map: AtoIMap,
    itoa_map: ItoAMap,
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl Alphabet {
    // builds the position/character associative arrays
    // also validates that the input alphabet doesn't have duplicates
    fn build_maps(alphabet: Vec<String>) -> Result<(AtoIMap, ItoAMap), Error> {
        let mut atoi: AtoIMap = HashMap::new();
        let mut itoa: ItoAMap = HashMap::new();

        for (i, c) in alphabet.into_iter().enumerate() {
            if atoi.contains_key(&c) {
                return Err(Error::ErrorDupChars);
            }
            atoi.insert(c.clone(), i);
            itoa.insert(i, c);
        }
        Ok((atoi, itoa))
    }

    pub fn contains(&self, c: String) -> bool {
        self.atoi_map.contains_key(&c)
    }
    /// checks that every char in key is in alphabet
    pub fn is_key_valid(&self, key: Key) -> bool {
        for c in key.value {
            if !self.contains(c) {
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
    fn add(&self, a: String, b: String) -> String {
        let a_pos = self.atoi_map.get(&a).unwrap();
        let b_pos = self.atoi_map.get(&b).unwrap();
        let new_pos = (a_pos + b_pos) % self.length;
        let c = self.itoa_map.get(&new_pos).unwrap();
        (*c).clone()
    }
    #[inline]
    fn sub(&self, a: String, b: String) -> String {
        self.add(a, self.inv(b))
    }
    #[inline]
    // return the additive inverse of a character given the alphabet
    fn inv(&self, c: String) -> String {
        let pos = self.atoi_map.get(&c).unwrap();
        let neg_pos = self.length - pos;
        let inv_c = self.itoa_map.get(&neg_pos).unwrap();
        (*inv_c).clone()
    }
}

impl TryFrom<String> for Alphabet {
    type Error = Error;
    /// Will Error if s contains duplicate characters or if s is shorter than 2
    fn try_from(s: String) -> Result<Self, Error> {
        if s.len() < 2 {
            return Err(Error::ErrorShortABC);
        }

        let value: Vec<String> = s
            .to_uppercase()
            .graphemes(true)
            .map(|s| s.to_string())
            .collect();

        let length = value.len();

        let (atoi_map, itoa_map) = Self::build_maps(value.clone())?;
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
    key: Key,
}
// Mode to know whether we are encrypting or decryptiong
pub enum Mode {
    /// Encrypt
    Encrypt,

    /// Decrypt
    Decrypt,
}

impl Vigenere {
    fn valid_key_or_err(key: &str, alphabet: &Alphabet) -> Result<(), Error> {
        if key.is_empty() {
            return Err(Error::ErrorShortKey);
        }
        if !alphabet.is_key_valid(key.into()) {
            return Err(Error::ErrorBadKeyChar);
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
            key: key.into(),
            alphabet,
        })
    }

    /// Decrypts or encrypts text depending on mode
    pub fn crypt(&self, text: &str, mode: Mode) -> Vec<String> {
        let key_mod = self.key.len();
        let mut key_pos: usize = 0;
        let mut vec: Vec<String> = Vec::with_capacity(text.len());

        let text = text.to_uppercase();
        for gr in text.graphemes(true) {
            let c = gr.to_string();
            if !self.alphabet.contains(c.clone()) {
                continue;
            }
            let key_char = self.key.char_from_position(key_pos);
            vec.push(match mode {
                Mode::Encrypt => self.alphabet.add(c, key_char),
                Mode::Decrypt => self.alphabet.sub(c, key_char),
            });
            key_pos += 1;
            key_pos %= key_mod;
        }
        vec
    }

    pub fn encrypt(&self, text: &str) -> Vec<String> {
        self.crypt(text, Mode::Encrypt)
    }
    pub fn decrypt(&self, text: &str) -> Vec<String> {
        self.crypt(text, Mode::Decrypt)
    }
}
